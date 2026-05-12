
Aura Edit (code-editor-Rust)
What is this?
Aura Edit is a high-performance code editor built from the ground up in Rust. While most modern editors rely on web technologies, this project uses Floem for the UI and leverages wgpu to handle rendering directly on the GPU. It’s designed using Rope Science to ensure that even massive files remain smooth and responsive.

I’m currently developing this project to explore high-performance GUI architecture and systems programming.

Quick Start & Installation
1. Prerequisites
To build Aura Edit from source, you need the Rust toolchain. If you don't have it yet, install it via rustup.rs.

For macOS (M4/M-Series) users:
Ensure you have the Xcode Command Line Tools installed:

xcode-select --install

2. Building from Source
Once Rust is installed, you can clone and run the editor immediately:

# Clone the repository
git clone https://github.com/AabhaJahagirdar/code-editor-Rust.git
cd code-editor-Rust

# Run in release mode for maximum performance
cargo run --release

3. Linux Dependencies
If you are on Linux, you may need to install the following development libraries:

libx11-dev

libwayland-dev

libasound2-dev

Key Features
Native GPU Rendering: Uses wgpu for a hardware-accelerated, high-FPS interface.

Intelligent Coding: Built-in LSP support provides autocompletion, diagnostics, and code actions.

Modal Editing: Vim-like modal editing is a first-class citizen and can be toggled easily.

Remote Development: Seamlessly work on remote systems with a local-speed experience.

WASM Plugin System: Write extensions in any language that compiles to WASI (C, Rust, AssemblyScript).

Integrated Terminal: Execute commands and manage your workspace without leaving the editor.

Architecture & TechnologyAura Edit is built on three main pillars:Floem UI: A native Rust UI toolkit that provides the layout and widget system.Rope Science: A data structure that allows for $O(\log n)$ text manipulation, making it superior for large files.WGPU: A cross-platform graphics API that allows the editor to run on Vulkan, Metal, and DirectX.