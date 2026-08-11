# VANTA v0.0.1

Keyboard-first terminal editor built from scratch in Rust.

This milestone intentionally has **zero third-party dependencies**.

## Current goal

Build a modal editor from the ground up instead of wrapping Vim/Neovim.

v0.0.1 provides:
- Windows console input through WinAPI FFI
- ANSI screen output
- keyboard event decoding
- a tiny event loop
- `q` to quit

## Run

Requirements:
- Windows
- Rust stable

```powershell
cargo run
```

Press any key to inspect the event. Press `q` to exit.

## Roadmap

1. terminal runtime
2. screen abstraction
3. cursor
4. text buffer
5. NORMAL / INSERT modes
6. motions and operators
7. undo/redo
8. search
9. splits and buffers
10. Tree-sitter
11. LSP
12. Git
13. terminal/PTY
14. Codex agent
