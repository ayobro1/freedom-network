@echo off
REM Freedom Network - Local VPN/Proxy Service
REM This script starts the Freedom Network node with HTTP proxy enabled
REM Configure your browser to use: 127.0.0.1:8080 as HTTP proxy

setlocal enabledelayedexpansion

echo.
echo ╔════════════════════════════════════════════════════════════════╗
echo ║         FREEDOM NETWORK - VPN/PROXY SERVICE                   ║
echo ╚════════════════════════════════════════════════════════════════╝
echo.

REM Navigate to the script directory
cd /d "%~dp0"

REM Check if the node binary exists
if not exist "node\target\release\freedom-node.exe" (
    echo.
    echo [ERROR] Binary not found. Building Freedom Network...
    echo.
    cd node
    cargo build --release
    cd ..
)

REM Start the VPN service
echo [*] Starting Freedom Network VPN service...
echo [*] The service will listen on: http://127.0.0.1:8080
echo.
echo [!] BROWSER CONFIGURATION REQUIRED [!]
echo.
echo For Firefox:
echo   1. Open Settings ^(about:preferences^)
echo   2. Scroll to Network Settings
echo   3. Under "Configure Proxy Access to the Internet", select "Manual proxy configuration"
echo   4. HTTP Proxy: 127.0.0.1
echo   5. Port: 8080
echo   ^(Optional: Enable "Proxy DNS when using SOCKS v5"^)
echo.
echo For Chrome/Edge:
echo   1. Open Settings
echo   2. Click "Advanced" ^(bottom left^)
echo   3. Open "System" menu
echo   4. Click "Open your computer's proxy settings"
echo   5. Under "Manual proxy setup" toggle "On"
echo   6. HTTP Proxy: 127.0.0.1
echo   7. Port: 8080
echo.
echo Press Ctrl+C to stop the service.
echo.
echo ════════════════════════════════════════════════════════════════
echo.

REM Run the node binary
node\target\release\freedom-node.exe

pause
