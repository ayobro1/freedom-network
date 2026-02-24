@echo off
REM Start the Freedom Browser in development mode

echo 🚀 Starting Freedom Browser (Development Mode)
echo.

cd /d "%~dp0"
cd src-tauri

echo 📦 Building and running Freedom Browser...
echo Make sure the QUIC node is running on 127.0.0.1:5000
echo.

cargo tauri dev

if %errorlevel% neq 0 (
    echo.
    echo ❌ Failed to start Freedom Browser
    echo.
    echo Make sure you have:
    echo  1. Tauri CLI installed: cargo install tauri-cli
    echo  2. Rust 1.70+ installed
    echo  3. The Freedom Network node running on 127.0.0.1:5000
    echo.
    pause
    exit /b 1
)
