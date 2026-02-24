@echo off
REM Build the Freedom Browser for release

echo 🔨 Building Freedom Browser (Release Mode)
echo.

cd /d "%~dp0"
cd src-tauri

echo 📦 Compiling for Release...
echo This may take a few minutes...
echo.

cargo tauri build

if %errorlevel% neq 0 (
    echo.
    echo ❌ Build failed
    echo.
    pause
    exit /b 1
)

echo.
echo ✅ Build complete!
echo.
echo 📍 Executable location:
echo %~dp0src-tauri\target\release\freedom-browser-tauri.exe
echo.
pause
