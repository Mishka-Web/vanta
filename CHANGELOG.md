# Changelog

## [0.0.2] - 2026-08-11

### Added
- GitHub Actions CI workflow for format, check, clippy, and tests.
- GitHub Actions release workflow for tagged Windows builds.
- Terminal size detection on Windows.
- `Screen` virtual framebuffer.
- `Cell` abstraction.
- Cursor state inside the screen.
- Screen resize support.
- Buffered rendering through ANSI output.
- Minimal diff-friendly renderer foundation.

### Changed
- UI rendering no longer writes the whole interface directly from `main.rs`.
- Rendering now flows through `Screen`.

## [0.0.1] - 2026-08-11

### Added
- Windows Console API input.
- ANSI output.
- Keyboard event loop.
- Basic key decoding.
