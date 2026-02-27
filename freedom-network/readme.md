# Freedom Network Workspace

This directory contains the core runtime components for the Freedom Network stack: node, browser, UI, content format tooling, and bundled example sites.

## Components

- `node/` Rust node runtime (networking, routing, identity, proxy, and site serving).
- `browser/` Rust browser client and chat UI modules.
- `fdom/` Parser and renderer for `.fdom` content.
- `app/` Tauri-based desktop application shell.
- `ui/` Lightweight local web dashboard for status and navigation.
- `sites/` Example Freedom sites used for testing and demos.
- `scripts/` Windows-oriented setup/build helper scripts.

## Quick Start

- Start the node service first.
- Launch the desktop app in `app/` as the primary control surface.
- Use **Connect** to start/stop the node directly from the desktop app.
- Choose VPN mode:
	- `Full Tunnel (System Proxy)`: routes system traffic through `127.0.0.1:8080` (Windows).
	- `App Only`: keeps routing scoped to app-level flows without system proxy changes.
- Optional local dashboard remains available at `http://127.0.0.1:9090` for diagnostics.

## Desktop VPN Options

The desktop app includes NordVPN-style operational options:

- Mode selection (`Full Tunnel` / `App Only`)
- Protocol preference (`Auto`, `QUIC`, `TCP`)
- Location profile selection (`Fastest`, `Nearest`, regional profiles)
- Kill Switch toggle
- Threat Protection toggle
- DNS Protection toggle
- Auto Connect toggle
- Auto-connect rule (`On Startup`, `Untrusted Wi-Fi`, `Off`)
- Split-tunneling app list (process names)
- Split-tunnel enforcement controls (`Apply Split Apps`, `Stop Split Apps`)
- Diagnostics run (dashboard/proxy/QUIC reachability + latency)
- Profile export/import (JSON)

### Kill Switch Behavior

- In `Full Tunnel` mode, disconnecting while Kill Switch is enabled keeps system proxy in a blocked state to prevent unprotected traffic leakage.
- Reconnecting restores routing to `127.0.0.1:8080`.

### Split-Tunneling Enforcement

- In `App Only` mode, entries in split-tunnel list are launched as managed processes with proxy environment variables set (`HTTP_PROXY`, `HTTPS_PROXY`, `ALL_PROXY`).
- `Apply Split Apps` restarts managed split-tunnel processes using the latest list.
- `Stop Split Apps` terminates all managed split-tunnel processes launched by the app.

### Strict System Interception (Windows)

- Enable `System Intercept (Windows)` to apply firewall-level rules for stronger leakage prevention and existing-process enforcement.
- In `Full Tunnel` mode, this can block outbound Internet globally while allowing node traffic.
- In `Single App` mode, listed split apps get direct-Internet block rules so they must use local proxy-based paths.
- This mode requires elevated permissions on Windows (PowerShell firewall rule changes).

## Windows Build + Installer

- Run `scripts\\windows-all-in-one-installer.bat` from Windows Command Prompt.
- The script stashes local changes, pulls latest changes from GitHub, reapplies the stash, and builds release binaries.
- It creates:
	- `dist-windows\\` (compiled artifacts)
	- `FreedomNetwork-Windows-AllInOne.zip` (single distributable package)

## Notes

- Most helper scripts in this repo target Windows (`.bat`).
- See `../README.md` for architecture context and project-level details.
