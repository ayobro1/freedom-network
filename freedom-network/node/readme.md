
### **Node**
- `node/src/main.rs` → Launch QUIC node
- `identity.rs` → Post-quantum key generation
- `encrypt.rs` → ChaCha20-Poly1305 encrypt/decrypt
- `routing.rs` → Multi-hop/onion routing (DHT)
- `Cargo.toml` → Rust dependencies

### **Browser**
- `main.rs` → Launch GUI
- `ui.rs` → Arc/Nook-style home screen and top sites
- `chat.rs` → Chat message structure & UI logic
- `network.rs` → QUIC client / multi-hop connections
- `Cargo.toml` → Rust dependencies

### **Sites**
- `chat-site/chat.fdom` → Main chat site
- `chat.js` → Chat logic (encrypt & send messages)
- `style.css` → Chat site styling
- Additional `.freedom` sites can be added as folders

### **Scripts**
- `setup.bat` → Build node/browser, generate keys
- `start-node.bat` → Launch QUIC node
- `start-browser.bat` → Launch Freedom Browser
- `package-site.js` → Encrypt/sign `.fdom` sites for sharing

---

## **Setup Instructions (Windows)**

1. **Clone repository:**
```powershell
git clone https://github.com/your-repo/freedom-network.git
cd freedom-network