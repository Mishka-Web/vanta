@echo off

powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0scripts\install.ps1"
if errorlevel 1 (
  echo.
  echo VANTA installation failed.
  exit /b %errorlevel%
)

set "VANTA_INSTALL_DIR=%LOCALAPPDATA%\Programs\VANTA"

echo;%PATH%; | find /I ";%VANTA_INSTALL_DIR%;" >nul
if errorlevel 1 (
  set "PATH=%PATH%;%VANTA_INSTALL_DIR%"
)

echo.
echo VANTA installation completed.
echo.
vanta --version
if errorlevel 1 (
  echo.
  echo VANTA was installed, but the current shell could not refresh PATH.
  echo Open a new terminal and run: vanta --version
  exit /b 0
)

echo.
echo VANTA is ready in this terminal.
