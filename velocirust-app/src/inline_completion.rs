use std::{borrow::Cow, ops::Range, path::PathBuf, str::FromStr};

use floem::reactive::{RwSignal, Scope, SignalGet, SignalUpdate, SignalWith, batch};
use velocirust_core::{
    buffer::{
        Buffer,
        rope_text::{RopeText, RopeTextRef},
    },
    rope_text_pos::RopeTextPosition,
    selection::Selection,
};
use lsp_types::InsertTextFormat;

use crate::{config::VelocirustConfig, doc::Doc, editor::EditorData, snippet::Snippet};

/// Redefinition of lsp types inline completion item with offset range
#[derive(Debug, Clone)]
pub struct InlineCompletionItem {
    pub insert_text: String,
    pub filter_text: Option<String>,
    pub range: Option<Range<usize>>,
    pub command: Option<lsp_types::Command>,
    pub insert_text_format: Option<InsertTextFormat>,
}

impl InlineCompletionItem {
    pub fn from_lsp(buffer: &Buffer, item: lsp_types::InlineCompletionItem) -> Self {
        let range = item.range.map(|r| {
            let start = buffer.offset_of_position(&r.start);
            let end = buffer.offset_of_position(&r.end);
            start..end
        });
        Self {
            insert_text: item.insert_text,
            filter_text: item.filter_text,
            range,
            command: item.command,
            insert_text_format: item.insert_text_format,
        }
    }

    pub fn apply(
        &self,
        editor: &EditorData,
        start_offset: usize,
    ) -> anyhow::Result<()> {
        let text_format = self
            .insert_text_format
            .unwrap_or(InsertTextFormat::PLAIN_TEXT);

        let selection = if let Some(range) = &self.range {
            Selection::region(range.start, range.end)
        } else {
            Selection::caret(start_offset)
        };

        match text_format {
            InsertTextFormat::PLAIN_TEXT => editor.do_edit(
                &selection,
                &[(selection.clone(), self.insert_text.as_str())],
            ),
            InsertTextFormat::SNIPPET => {
                editor.completion_apply_snippet(
                    &self.insert_text,
                    &selection,
                    Vec::new(),
                    start_offset,
                )?;
            }
            _ => {}
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlineCompletionStatus {
    Inactive,
    Started,
    Active,
}

#[derive(Clone, Debug)]
pub struct InlineCompletionData {
    // These are now signals to allow modification via &self (Interior Mutability)
    pub status: RwSignal<InlineCompletionStatus>,
    pub items: RwSignal<im::Vector<InlineCompletionItem>>,
    pub active: RwSignal<usize>,
    pub start_offset: RwSignal<usize>,
    pub path: RwSignal<PathBuf>,
    pub is_fetching: RwSignal<bool>,
}

impl InlineCompletionData {
    pub fn new(cx: Scope) -> Self {
        Self {
            status: cx.create_rw_signal(InlineCompletionStatus::Inactive),
            active: cx.create_rw_signal(0),
            items: cx.create_rw_signal(im::vector![]),
            start_offset: cx.create_rw_signal(0),
            path: cx.create_rw_signal(PathBuf::new()),
            is_fetching: cx.create_rw_signal(false),
        }
    }

    pub fn current_item(&self) -> Option<InlineCompletionItem> {
        let active = self.active.get_untracked();
        self.items.with_untracked(|items| items.get(active).cloned())
    }

    pub fn next(&self) {
        let len = self.items.with_untracked(|i| i.len());
        if len > 0 {
            let next_index = (self.active.get_untracked() + 1) % len;
            self.active.set(next_index);
        }
    }

    pub fn previous(&self) {
        let len = self.items.with_untracked(|i| i.len());
        if len > 0 {
            let current = self.active.get_untracked();
            let prev_index = if current == 0 { len - 1 } else { current - 1 };
            self.active.set(prev_index);
        }
    }

    pub fn cancel(&self) {
        if self.status.get_untracked() == InlineCompletionStatus::Inactive {
            return;
        }

        batch(|| {
            self.items.update(|i| i.clear());
            self.status.set(InlineCompletionStatus::Inactive);
            self.is_fetching.set(false);
        });
    }

    pub fn set_items(
        &self,
        items: im::Vector<InlineCompletionItem>,
        start_offset: usize,
        path: PathBuf,
    ) {
        batch(|| {
            self.items.set(items);
            self.active.set(0);
            self.status.set(InlineCompletionStatus::Active);
            self.start_offset.set(start_offset);
            self.path.set(path);
            self.is_fetching.set(false);
        });
    }

    pub fn update_doc(&self, doc: &Doc, offset: usize) {
        if self.status.get_untracked() != InlineCompletionStatus::Active {
            doc.clear_inline_completion();
            return;
        }

        let items_len = self.items.with_untracked(|i| i.len());
        if items_len == 0 {
            doc.clear_inline_completion();
            return;
        }

        let active = self.active.get_untracked();
        let active = if active >= items_len {
            self.active.set(0);
            0
        } else {
            active
        };

        self.items.with_untracked(|items| {
            if let Some(item) = items.get(active) {
                let text = item.insert_text.clone();
                let offset = item.range.as_ref().map(|r| r.start).unwrap_or(offset);
                let (line, col) = doc
                    .buffer
                    .with_untracked(|buffer| buffer.offset_to_line_col(offset));
                doc.set_inline_completion(text, line, col);
            }
        });
    }

    pub fn update_inline_completion(
        &self,
        config: &VelocirustConfig,
        doc: &Doc,
        cursor_offset: usize,
    ) {
        if !config.editor.enable_inline_completion {
            doc.clear_inline_completion();
            return;
        }

        let text = doc.buffer.with_untracked(|buffer| buffer.text().clone());
        let text = RopeTextRef::new(&text);
        let Some(item) = self.current_item() else {
            return;
        };

        let start_offset = self.start_offset.get_untracked();
        let completion = doc.inline_completion.with_untracked(|cur| {
            let cur = cur.as_deref();
            inline_completion_text(text, start_offset, cursor_offset, &item, cur)
        });

        match completion {
            ICompletionRes::Hide => {
                doc.clear_inline_completion();
            }
            ICompletionRes::Unchanged => {}
            ICompletionRes::Set(new, shift) => {
                let offset = start_offset + shift;
                let (line, col) = text.offset_to_line_col(offset);
                doc.set_inline_completion(new, line, col);
            }
        }
    }
}

enum ICompletionRes {
    Hide,
    Unchanged,
    Set(String, usize),
}

fn inline_completion_text(
    rope_text: impl RopeText,
    start_offset: usize,
    cursor_offset: usize,
    item: &InlineCompletionItem,
    current_completion: Option<&str>,
) -> ICompletionRes {
    let text_format = item
        .insert_text_format
        .unwrap_or(InsertTextFormat::PLAIN_TEXT);

    let cursor_prev_offset = rope_text.prev_code_boundary(cursor_offset);
    if let Some(range) = &item.range {
        let edit_start = range.start;

        if cursor_prev_offset != edit_start && start_offset != edit_start {
            return ICompletionRes::Hide;
        }
    }

    let text = match text_format {
        InsertTextFormat::PLAIN_TEXT => Cow::Borrowed(&item.insert_text),
        InsertTextFormat::SNIPPET => {
            let Ok(snippet) = Snippet::from_str(&item.insert_text) else {
                return ICompletionRes::Hide;
            };
            Cow::Owned(snippet.text())
        }
        _ => {
            return ICompletionRes::Hide;
        }
    };

    let range = start_offset..rope_text.offset_line_end(start_offset, true);
    let prefix = rope_text.slice_to_cow(range);
    
    let Some(text) = text.strip_prefix(prefix.as_ref()) else {
        return ICompletionRes::Hide;
    };

    if Some(text) == current_completion {
        ICompletionRes::Unchanged
    } else {
        ICompletionRes::Set(text.to_string(), prefix.len())
    }
}