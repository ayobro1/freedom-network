# 🌐 Freedom Network

**Level 4 Decentralized Network | Post-Quantum Encryption | Custom Sovereign Browser**

The Freedom Network is a fully decentralized, privacy-first ecosystem featuring a high-performance routing node and a custom-built browser. It leverages **QUIC transport** and **Post-Quantum Cryptography (PQC)** to provide a browsing experience entirely independent of the traditional web.

---

## 🏗 System Architecture

### 1. ⚡ The Node (`/node`)
The backbone of the network. Built in **Rust**, it handles the heavy lifting of peer-to-peer communication.
* **QUIC Transport:** Low-latency, encrypted-by-default connections.
* **PQC Identity:** Key generation resistant to future quantum computing threats.
* **Onion Routing:** Multi-hop path selection to obfuscate traffic origin and destination.

### 2. 🧊 The Browser (`/browser`)
A sovereign GUI built with the **Iced** framework, operating without Chromium or Webkit dependencies.
* **Sleek Interface:** Modern, productivity-focused layout with integrated navigation.
* **Native .fdom Rendering:** Directly decrypts and renders proprietary network files in memory.
* **Integrated P2P Chat:** Real-time messaging functionality baked into the browser core.

### 3. 📄 Freedom Sites (`/sites`)
Content on the network is packaged into secure `.fdom` files.
* **Chat-Site Example:** A pre-configured decentralized messaging hub.
* **Encrypted Assets:** CSS and JS are signed and verified before execution.

---

## 📁 Repository Structure

```text
freedom-network/
├── node/                # Rust: QUIC node, PQC encryption, DHT routing
├── browser/             # Rust: Iced-based UI, .fdom renderer, P2P logic
├── sites/               # Hosted .freedom content (e.g., chat-site)
├── scripts/             # Automation: setup.bat, package-site.js
└── README.md            # You are here
