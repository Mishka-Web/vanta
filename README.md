# VANTA v0.0.3

Keyboard-first modal editor built from scratch in Rust.

This milestone still uses zero third-party dependencies.

## v0.0.3 — Input Engine

Structured keyboard input:

- KeyCode
- KeyModifiers
- Ctrl / Alt / Shift
- Tab / Delete / Insert
- Home / End
- PageUp / PageDown
- F1-F12
- repeat count

## Run

```powershell
cargo run
```

Plain `q` exits.

## Checks

```powershell
cargo fmt --all -- --check
cargo check
cargo clippy -- -D warnings
cargo test
```
