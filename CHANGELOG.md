# Changelog

## [0.0.5] - 2026-08-11

### Added
- `vanta --version`.
- `vanta --help`.
- Windows installer and uninstaller.
- `install.cmd` / `uninstall.cmd` wrappers for PowerShell execution-policy compatibility.
- Installer self-verification after copying `vanta.exe`.
- User PATH installation.
- GitHub Release packaging with `vanta.exe`.

### Fixed
- Standalone Shift/Ctrl/Alt key events no longer appear as `Unknown(...)`.
- Removed obsolete `Screen::move_cursor_*` methods so strict Clippy builds pass cleanly.
- `install.cmd` now refreshes PATH in the current CMD session after installation.

## [0.0.4] - 2026-08-11

### Added
- Editor cursor model.
- Viewport abstraction.
- `h/j/k/l` and arrow navigation.
- Home/End.
- Preferred column.
- Vertical scrolling.
- Line numbers and status line.

### Fixed
- Added missing `Screen::set_cursor`.

## [0.0.3] - 2026-08-11

### Added
- Structured input engine.
- Ctrl/Alt/Shift modifiers.
- Navigation and function keys.

## [0.0.2] - 2026-08-11

### Added
- Virtual screen engine.
- Terminal size and resize support.

## [0.0.1] - 2026-08-11

### Added
- Windows terminal runtime.
- Keyboard input.
- ANSI output.
