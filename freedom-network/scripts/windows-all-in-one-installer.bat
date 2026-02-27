@echo off
setlocal EnableExtensions EnableDelayedExpansion

echo.
echo ================================================
echo   Freedom Network - Windows All-in-One Build
echo ================================================
echo.

cd /d "%~dp0\.."
set "ROOT=%CD%"
set "DIST_DIR=%ROOT%\dist-windows"
set "STASHED=0"
set "SKIP_SYNC=0"

if /I "%~1"=="--no-sync" set "SKIP_SYNC=1"
if defined GITHUB_ACTIONS set "SKIP_SYNC=1"

where git >nul 2>&1
if errorlevel 1 (
    echo [ERROR] Git is not installed or not available in PATH.
    exit /b 1
)

where cargo >nul 2>&1
if errorlevel 1 (
    echo [ERROR] Rust/Cargo is not installed or not available in PATH.
    echo Install from https://rustup.rs and run this script again.
    exit /b 1
)

if "%SKIP_SYNC%"=="0" (
    for /f "delims=" %%B in ('git branch --show-current') do set "BRANCH=%%B"
    if "%BRANCH%"=="" (
        echo [ERROR] Could not determine current git branch.
        exit /b 1
    )

    echo [1/7] Saving local changes (if any)...
    for /f "delims=" %%S in ('git stash push -u -m "windows-all-in-one-auto-stash"') do set "STASH_OUTPUT=%%S"
    echo !STASH_OUTPUT! | findstr /C:"No local changes to save" >nul
    if errorlevel 1 (
        set "STASHED=1"
        echo        Local changes stashed.
    ) else (
        echo        No local changes found.
    )

    echo [2/7] Pulling latest changes from GitHub for branch %BRANCH%...
    git fetch origin
    if errorlevel 1 (
        echo [ERROR] git fetch failed.
        goto :restore_and_fail
    )

    git pull --rebase origin %BRANCH%
    if errorlevel 1 (
        echo [ERROR] git pull --rebase failed.
        goto :restore_and_fail
    )

    echo [3/7] Reapplying stashed changes (if any)...
    if "%STASHED%"=="1" (
        git stash pop
        if errorlevel 1 (
            echo [ERROR] Failed to reapply stashed changes. Resolve conflicts and retry.
            exit /b 1
        )
        echo        Changes reapplied successfully.
    ) else (
        echo        Nothing to reapply.
    )
) else (
    echo [SYNC] Skipping git stash/pull (CI mode or --no-sync).
)

echo [4/7] Ensuring Tauri CLI is installed...
cargo tauri --version >nul 2>&1
if errorlevel 1 (
    cargo install tauri-cli --locked
    if errorlevel 1 (
        echo [ERROR] Failed to install tauri-cli.
        exit /b 1
    )
)

echo [5/7] Building node executable...
cd /d "%ROOT%\node"
cargo build --release
if errorlevel 1 (
    echo [ERROR] Node build failed.
    exit /b 1
)

echo [6/7] Building desktop executable and NSIS installer...
cd /d "%ROOT%\app\src-tauri"
cargo tauri build
if errorlevel 1 (
    echo [ERROR] Tauri build failed.
    exit /b 1
)

echo [7/7] Packaging all artifacts...
if exist "%DIST_DIR%" rmdir /s /q "%DIST_DIR%"
mkdir "%DIST_DIR%"

if exist "%ROOT%\node\target\release\node.exe" copy /y "%ROOT%\node\target\release\node.exe" "%DIST_DIR%\" >nul
if exist "%ROOT%\app\src-tauri\target\release\freedom-browser-tauri.exe" copy /y "%ROOT%\app\src-tauri\target\release\freedom-browser-tauri.exe" "%DIST_DIR%\" >nul

for %%F in ("%ROOT%\app\src-tauri\target\release\bundle\nsis\*.exe") do copy /y "%%~fF" "%DIST_DIR%\" >nul
for %%F in ("%ROOT%\app\src-tauri\target\release\bundle\msi\*.msi") do copy /y "%%~fF" "%DIST_DIR%\" >nul

if exist "%ROOT%\scripts\start-node.bat" copy /y "%ROOT%\scripts\start-node.bat" "%DIST_DIR%\" >nul
if exist "%ROOT%\scripts\start-browser-ui.bat" copy /y "%ROOT%\scripts\start-browser-ui.bat" "%DIST_DIR%\" >nul

powershell -NoProfile -ExecutionPolicy Bypass -Command "Compress-Archive -Path '%DIST_DIR%\*' -DestinationPath '%ROOT%\FreedomNetwork-Windows-AllInOne.zip' -Force"
if errorlevel 1 (
    echo [ERROR] Failed to create zip package.
    exit /b 1
)

echo.
echo Build and packaging complete.
echo Output folder: %DIST_DIR%
echo All-in-one zip: %ROOT%\FreedomNetwork-Windows-AllInOne.zip
echo.
exit /b 0

:restore_and_fail
if "%STASHED%"=="1" (
    echo Attempting to restore stashed local changes...
    git stash pop >nul 2>&1
)
exit /b 1
