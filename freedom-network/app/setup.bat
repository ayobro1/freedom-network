@echo off
REM Install or verify Tauri CLI is available

echo 🔧 Checking for Tauri CLI...
cargo tauri --version > nul 2>&1

if %errorlevel% neq 0 (
    echo ⚠️  Tauri CLI not found. Installing...
    cargo install tauri-cli
    if %errorlevel% neq 0 (
        echo ❌ Failed to install Tauri CLI
        pause
        exit /b 1
    )
)

echo ✅ Tauri CLI is ready
echo.
echo 📝 To build and run the app:
echo    cargo tauri dev
echo.
pause
