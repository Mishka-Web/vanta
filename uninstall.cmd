@echo off

powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0scripts\uninstall.ps1"
if errorlevel 1 exit /b %errorlevel%

set "VANTA_INSTALL_DIR=%LOCALAPPDATA%\Programs\VANTA"
set "NEW_PATH="

for %%P in ("%PATH:;=" "%") do (
  if /I not "%%~P"=="%VANTA_INSTALL_DIR%" (
    if defined NEW_PATH (
      call set "NEW_PATH=%%NEW_PATH%%;%%~P"
    ) else (
      set "NEW_PATH=%%~P"
    )
  )
)

if defined NEW_PATH set "PATH=%NEW_PATH%"

echo VANTA removed from the current CMD session.
