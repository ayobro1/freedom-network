# 🌐 Freedom Network

**Level 4 Decentralized Network | Post-Quantum Encryption | Sovereign Web Ecosystem**

Freedom Network is a privacy-first, fully decentralized ecosystem designed to bypass the limitations and surveillance of the traditional web. It consists of a high-performance routing node and a custom-built browser that operates independently of Chromium or WebKit engines.

---

## 🚀 Key Features

* **Post-Quantum Cryptography (PQC):** Identity and encryption protocols built to withstand future quantum computing threats.
* **QUIC Transport:** Low-latency, encrypted-by-default connections for resilient peer-to-peer communication.
* **Sovereign Browser:** A native GUI built with the **Iced** framework, removing heavy dependencies on centralized browser engines.
* **Native .fdom Rendering:** Support for proprietary `.fdom` files—decentralized "sites" that are decrypted and rendered entirely in memory.
* **Onion Routing:** Multi-hop path selection to obfuscate traffic origin and destination metadata.

---

## 🏗 System Architecture

The project is structured into three primary components to ensure modularity and security:

### 1. The Node (`/node`)
The backbone of the network, written in **Rust**.
* Handles peer discovery and DHT (Distributed Hash Table) routing.
* Manages encrypted tunnels using the **QUIC** protocol.
* Performs PQC key exchanges for secure, long-term communication.

### 2. The Browser (`/browser`)
A lightweight, modern interface built using the **Iced** framework.
* **Native Rendering:** Directly renders `.fdom` content without a standard web engine.
* **Integrated P2P Chat:** Real-time messaging functionality baked directly into the browser core.
* **Privacy First:** No cookies, no tracking, and no centralized DNS lookups.

### 3. Freedom Sites (`/sites`)
Content packages optimized for the Freedom Network.
* Includes examples like a decentralized chat-site.
* Assets (CSS/JS) are signed and verified to prevent middle-man injection or tampering.

---

## 📁 Repository Structure

```text
freedom-network/
├── node/                # Rust: QUIC node, PQC encryption, DHT routing
├── browser/             # Rust: Iced-based UI, .fdom renderer, P2P logic
├── sites/               # Hosted .freedom content (e.g., chat-site)
├── scripts/             # Automation: setup.bat, package-site.js
├── ARCHITECTURE.md      # Detailed technical specifications
└── README.md            # Project overview
