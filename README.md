# VelociRust

## What is VelociRust?

VelociRust builds upon the high-performance foundations of Lapce to experiment with localized, real-time developer tools. While most modern editors rely on web-tech resource-heavy shells, this architecture uses the **Floem UI framework** and leverages `wgpu` to handle hardware-accelerated rendering directly on the GPU. 

### My Core Contribution: The Hybrid AI Submodule
The primary objective of this fork is the implementation of an experimental, thread-safe **Hybrid AI Autocompletion System**:
* **Local Edge Inference:** Integrates a thread-safe, background async worker that communicates with a local **DeepSeek-R1 (1.5B)** model via an Ollama pipeline.
* **Asynchronous AI Debouncer:** Implements a custom `DebouncedListener` that monitors keystroke pauses (150ms) to trigger predictive ghost-text generation without blocking the main rendering loop or UI thread.
* **Cloud Reasoning Bridge:** Hooks into cloud-based LLMs (like Gemini via the ADK-Gemini crate) to handle heavy-lifting tasks like complete file refactoring and structural explanations.

## Key Features

* **Experimental Hybrid AI:** Intelligent, low-latency code suggestions generated on the fly by local edge models, paired with deep cloud reasoning.
* **Native GPU Rendering:** Powered by `wgpu` for a hardware-accelerated, fluid interface utilizing Metal, Vulkan, or DirectX.
* **Rope Science Core:** Utilizes underlying rope data structures to guarantee $O(\log n)$ text manipulation, keeping editing smooth even with massive files.
* **Intelligent Coding:** Native Language Server Protocol (LSP) support providing autocompletion, diagnostics, and context-aware code actions.
* **Modal Editing:** First-class Vim-like modal navigation that can be toggled on demand.
* **WASM Plugin System:** Extensible via isolated sub-modules that compile down to WASI (Rust, C, AssemblyScript).
* **Integrated Terminal:** Built-in terminal simulation to execute system commands without leaving the GUI environment.

## Quick Start & Installation

### 1. Prerequisites
To build VelociRust from source, you need the standard Rust toolchain (`rustup.rs`).

* **For macOS (M-Series/M4) users:** Ensure Xcode Command Line Tools are active:
  ```bash
  xcode-select --install

* **For Local AI Capabilities: Install Ollama and pull the completion model:
  ```bash
  ollama run deepseek-r1:1.5b

* **Building from Source
Clone the repository and run the application wrapper:

  ```bash
  git clone [https://github.com/AabhaJahagirdar/VelociRust.git]   (https://github.com/AabhaJahagirdar/VelociRust.git)
  cd VelociRust
  cargo run --release

* **Linux Dependencies
If compiling on a Linux machine, ensure the following native package dependencies are configured:
  ```bash
  # Ubuntu/Debian example
  sudo apt install libx11-dev libwayland-dev libasound2-dev

* **Architecture Foundations
  
VelociRust operates on three structural pillars inherited from the upstream Lapce ecosystem:
1) Floem UI: A reactive, native Rust widget and layout system built specifically for smooth performance.
2) Rope Data Structures: Replaces traditional flat array string buffers with node trees, ensuring editing multi-megabyte source files doesn't stall the thread.
3) Cross-Platform Graphics: Bypasses web-view overhead by compiling instructions straight into GPU operations.

⚠️ **Important Notice & Credits:** This repository is an educational, non-commercial fork of the open-source [Lapce Text Editor](https://github.com/lapce/lapce). It is built on top of Lapce's original source code architecture as part of a college engineering mini-project to explore systems programming and high-performance GUI development. All original codebase credits, core editor features, and architectural authorship belong entirely to the Lapce team and its open-source contributors.
