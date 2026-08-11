# VANTA v0.0.5

Keyboard-first modal editor being built from scratch in Rust.

## v0.0.5 — Distribution / Installer

VANTA can now be installed and invoked as a normal Windows CLI product.

### Build

```powershell
cargo build --release
```

### Install from the repository

```cmd
install.cmd
```

`install.cmd` launches the PowerShell installer with a process-local execution-policy bypass, so the user does not need to change the machine or user PowerShell policy.

Open a new terminal:

```powershell
vanta --version
vanta --help
vanta
```

Default location:

```text
%LOCALAPPDATA%\Programs\VANTA\vanta.exe
```

The installer adds that folder to the current user's persistent PATH. When launched from CMD via `install.cmd`, it also refreshes PATH in the current CMD session.

### Uninstall

```cmd
uninstall.cmd
```

### GitHub Release

Pushing a `v*` tag builds `vanta.exe` and publishes:

```text
vanta-windows-x64.zip
```

The release ZIP contains the executable, installer, uninstaller, README and changelog.
The end user does not need Rust or Cargo.

## Editor controls

```text
h / Left      move left
j / Down      move down
k / Up        move up
l / Right     move right
Home          beginning of line
End           end of line
q             quit
```
