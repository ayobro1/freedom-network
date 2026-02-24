@echo off
echo.
echo ========================================
echo  🌐 Freedom Network - Complete Build
echo ========================================
echo.

echo Building Freedom Node...
cd node
cargo build --release
if %ERRORLEVEL% NEQ 0 (
    echo Build failed!
    pause
    exit /b 1
)
cd ..

echo.
echo ✓ Node built successfully!
echo.

echo.
echo ========================================
echo  Network Compiled and Ready!
echo ========================================
echo.
echo To run the network:
echo.
echo 1. Start the node:
echo    .\scripts\start-node.bat
echo.
echo 2. Open the browser:
echo    Open http://localhost in your browser
echo    (or manually open freedom-network\ui\index.html)
echo.
echo 3. The browser can now:
echo    - Click tabs to navigate sites
echo    - Type freedom:// addresses
echo    - Fetch content through P2P network
echo.
echo Architecture Details:
echo - Read ARCHITECTURE.md for full protocol documentation
echo - DHT-based .freedom domain registration
echo - Multi-hop routing like Tor
echo - Decentralized content distribution
echo.
echo Happy Freedom Networking!
echo.
pause
