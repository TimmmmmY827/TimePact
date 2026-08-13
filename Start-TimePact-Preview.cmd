@echo off
setlocal
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0scripts\Start-TimePact-Preview.ps1"
set "preview_exit_code=%ERRORLEVEL%"
if not "%preview_exit_code%"=="0" (
  echo.
  echo TimePact Preview failed to start.
  pause
)
exit /b %preview_exit_code%
