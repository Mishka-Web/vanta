# VANTA v0.0.2

Keyboard-first terminal editor built from scratch in Rust.

This milestone intentionally has **zero third-party dependencies**.

## What's new in 0.0.2

The editor now has a real virtual screen layer:

```text
Input
  ↓
App
  ↓
Screen
  ├── Cell[]
  ├── width
  ├── height
  └── cursor
  ↓
Terminal renderer
```

Current controls:
- any key: show key event
- arrows: move the virtual cursor
- `q`: quit

## Run

```powershell
cargo run
```

Windows only for now.
