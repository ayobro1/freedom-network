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
- Launch either the browser app (`browser/`) or web UI (`ui/server.py`).
- Open `http://127.0.0.1:8000` for the local dashboard.
- Ensure API endpoints are reachable at `http://127.0.0.1:9090` for live stats.

## Windows Build + Installer

- Run `scripts\\windows-all-in-one-installer.bat` from Windows Command Prompt.
- The script stashes local changes, pulls latest changes from GitHub, reapplies the stash, and builds release binaries.
- It creates:
	- `dist-windows\\` (compiled artifacts)
	- `FreedomNetwork-Windows-AllInOne.zip` (single distributable package)

## Notes

- Most helper scripts in this repo target Windows (`.bat`).
- See `../README.md` for architecture context and project-level details.
