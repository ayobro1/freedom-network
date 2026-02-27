# Freedom Network

**Desktop-First Private Routing | Decentralized Node | Secure Local Tunneling**

Freedom Network is a privacy-first ecosystem for local secure tunneling and decentralized routing. It includes a Rust node runtime, a desktop Tauri control app, and supporting browser/content tooling.

---

## Key Features

* **Post-Quantum Cryptography (PQC):** Identity and encryption protocols built to withstand future quantum computing threats.
* **QUIC Transport:** Low-latency, encrypted-by-default connections for resilient peer-to-peer communication.
* **Desktop VPN Control App:** Native Tauri application with one-click connect/disconnect, diagnostics, split tunneling, and profile import/export.
* **Native .fdom Rendering:** Support for proprietary `.fdom` files—decentralized "sites" that are decrypted and rendered entirely in memory.
* **Onion Routing:** Multi-hop path selection to obfuscate traffic origin and destination metadata.

---

## System Architecture

The project is structured into three primary components to keep networking, desktop UX, and content tooling modular:

### 1. The Node (`/node`)
The backbone of the network, written in **Rust**.
* Handles peer discovery and DHT (Distributed Hash Table) routing.
* Manages encrypted tunnels using the **QUIC** protocol.
* Performs PQC key exchanges for secure, long-term communication.

### 2. The Desktop App (`/app`)
A Tauri-based desktop VPN controller and local diagnostics UI.
* Start/stop node runtime from the app.
* Configure full-tunnel vs app-only routing modes.
* Apply Windows interception mode, kill switch, and split tunnel controls.

### 3. Browser + Sites (`/browser`, `/sites`)
Content packages optimized for the Freedom Network.
* Includes examples like a decentralized chat-site.
* Assets (CSS/JS) are signed and verified to prevent middle-man injection or tampering.

---

## Installation

### Prerequisites

* **Git**
* **Rust (stable toolchain + Cargo):** https://rustup.rs
* **Windows only (desktop app packaging):** PowerShell and standard Visual Studio C++ build tools

### Windows (All-in-One Build + Package)

From the repository root:

```bat
cd freedom-network
scripts\windows-all-in-one-installer.bat
```

This script will:

* Pull latest changes (unless run in CI or with `--no-sync`)
* Build release binaries
* Build the desktop installer
* Produce:
	* `freedom-network\dist-windows\`
	* `freedom-network\FreedomNetwork-Windows-AllInOne.zip`

### Manual Development Setup (Any OS)

```bash
cd freedom-network/node
cargo build --release

cd ../app/src-tauri
cargo tauri dev
```

Optional: build the legacy Rust browser client:

```bash
cd ../browser
cargo build --release
```

Optional local web dashboard:

```bash
cd ../ui
python3 server.py
```

---

##  Repository Structure

```text
freedom-network/
├── node/                # Rust node runtime: proxy + routing + dashboard APIs
├── app/                 # Tauri desktop VPN control app (primary UX)
├── browser/             # Legacy Rust browser client modules
├── sites/               # Example content packages
├── scripts/             # Build/packaging automation
├── ARCHITECTURE.md      # Technical architecture notes
└── README.md            # Project overview
