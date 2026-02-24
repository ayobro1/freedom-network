@echo off
echo.
echo ============================================
echo   🌐 Freedom Browser Web Interface
echo ============================================
echo.
cd /d "%~dp0ui"
python server.py
pause
