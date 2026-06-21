use std::time::Duration;
use floem::reactive::{RwSignal, Scope, SignalGet, SignalUpdate};
use floem::action::exec_after;

/// A signal listener that receives 'events' from the outside and runs the callback.  
#[derive(Debug)]
pub struct Listener<T: 'static> {
    cx: Scope,
    val: RwSignal<Option<T>>,
}

// Manual implementation for flexibility with generic T
impl<T: 'static> Clone for Listener<T> {
    fn clone(&self) -> Self { *self }
}
impl<T: 'static> Copy for Listener<T> {}

impl<T: Clone + 'static> Listener<T> {
    pub fn new(cx: Scope, on_val: impl Fn(T) + 'static) -> Listener<T> {
        let val = cx.create_rw_signal(None);
        let listener = Listener { val, cx };
        listener.listen(on_val);
        listener
    }

    pub fn new_empty(cx: Scope) -> Listener<T> {
        let val = cx.create_rw_signal(None);
        Listener { val, cx }
    }

    pub fn scope(&self) -> Scope { self.cx }

    pub fn listen(self, on_val: impl Fn(T) + 'static) {
        self.listen_with(self.cx, on_val)
    }

    pub fn listen_with(self, cx: Scope, on_val: impl Fn(T) + 'static) {
        let val = self.val;
        cx.create_effect(move |_| {
            if let Some(cmd) = val.get() {
                on_val(cmd);
            }
        });
    }

    pub fn send(&self, v: T) {
        self.val.set(Some(v));
    }
}

/// A specialized listener for AI completions
#[derive(Clone, Debug)] // This now works because Listener is Debug/Clone
pub struct DebouncedListener<T: Clone + 'static> {
    pub listener: Listener<T>,
    pub delay: Duration,
    pub active_timer: RwSignal<Option<floem::action::TimerToken>>,
}

impl<T: Clone + 'static> DebouncedListener<T> {
    pub fn new(cx: Scope, delay_ms: u64, on_val: impl Fn(T) + 'static) -> Self {
        let listener = Listener::new(cx, on_val);
        let active_timer = cx.create_rw_signal(None);

        Self {
            listener,
            delay: Duration::from_millis(delay_ms),
            active_timer,
        }
    }

    pub fn send_debounced(&self, v: T) {
        let listener = self.listener;
        let active_timer = self.active_timer;
        active_timer.set(None);
        let timer = exec_after(self.delay, move |_| {
            listener.send(v);
        });
        active_timer.set(Some(timer));
    }
}