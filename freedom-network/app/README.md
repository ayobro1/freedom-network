# Freedom Browser - Desktop Application

A standalone Tauri-based desktop browser for the Freedom Network P2P system.

## What is Freedom Browser?

Freedom Browser is a native desktop application that lets you browse the decentralized Freedom Network, similar to how Tor Browser works but for `.freedom` domains.

## Features

✅ **Standalone Desktop App** - Native window, not a web page
✅ **Multi-tab Navigation** - Tab-based interface (Arc-inspired sidebar)
✅ **Address Bar** - Type `.freedom` domains to navigate
✅ **Connected to Node** - Communicates with the Rust QUIC backend on `127.0.0.1:5000`
✅ **Dark Theme** - Modern dark UI with purple/blue accent colors

## Architecture

```
┌─────────────────────────────────────┐
│  Freedom Browser (Desktop App)      │
│  ┌──────────────────────────────┐   │
│  │  Tauri Runtime               │   │
│  ├──────────────────────────────┤   │
│  │  HTML/CSS/JS Frontend        │   │
│  │  - Tab switching             │   │
│  │  - Address bar navigation    │   │
│  └──────────────────────────────┘   │
│              ↕                       │
│  ┌──────────────────────────────┐   │
│  │ Tauri IPC Bridge (Commands)  │   │
│  └──────────────────────────────┘   │
└─────────────────────────────────────┘
              ↕
    Freedom Network Node
    (127.0.0.1:5000)
    - DHT
    - Routing
    - Site Server
```

## Building

### Requirements

- Rust 1.70+
- Tauri CLI

### Install Tauri CLI

```powershell
cargo install tauri-cli
```

### Build the App

```powershell
cd app/src-tauri
cargo tauri build
```

The compiled executable will be at:
```
app/src-tauri/target/release/freedom-browser-tauri.exe
```

### Run in Development

```powershell
cd app/src-tauri
cargo tauri dev
```

## Usage

1. Make sure the Freedom Network Node is running on `127.0.0.1:5000`:
   ```powershell
   cd ../../../node
   cargo run
   ```

2. Launch Freedom Browser:
   ```powershell
   cd ../../app/src-tauri
   cargo tauri dev
   ```

3. Navigate using:
   - Click tabs on the left sidebar
   - Type addresses in the address bar: `freedom://chat`, `freedom://example`
   - Press Enter or click the arrow button

## Project Structure

```
app/
├── src/                          # Web UI files
│   ├── index.html               # Main application window
│   ├── style.css                # Dark theme styling
│   └── app.js                   # Tab logic & UI interactions
├── src-tauri/                   # Rust/Tauri backend
│   ├── src/
│   │   └── main.rs              # Tauri app setup & IPC commands
│   ├── build.rs                 # Build script
│   ├── Cargo.toml               # Tauri dependencies
│   └── tauri.conf.json          # Tauri configuration
└── Cargo.toml
```

## IPC Commands

The Tauri app exposes these commands to the frontend:

### `fetch_freedom_site(domain, path)`
Fetch content from a `.freedom` domain via the Freedom Network node.

```javascript
const content = await invoke('fetch_freedom_site', {
    domain: 'example',
    path: '/index.html'
});
```

### `get_node_status()`
Check the status of the connected Freedom Network node.

```javascript
const status = await invoke('get_node_status');
```

## Customization

### Change Window Size

Edit `app/src-tauri/tauri.conf.json`:

```json
"windows": [
  {
    "width": 1200,
    "height": 800,
    "minWidth": 800,
    "minHeight": 600
  }
]
```

### Change App Icon

Place your icon at `app/src-tauri/icons/icon.ico` and update `tauri.conf.json`.

### Change Theme Colors

Edit `app/src/style.css` CSS variables:

```css
:root {
    --accent-blue: #4a9eff;
    --accent-purple: #7c3aed;
    --bg-primary: #0a0e27;
}
```

## Security Considerations

- The app communicates with the local node on `127.0.0.1:5000`
- All network connections go through the Freedom Network (QUIC)
- No telemetry or external connections by default
- Content is verified cryptographically by the node

## Troubleshooting

### "Failed to build Tauri app"
Make sure you have Tauri CLI installed:
```powershell
cargo install tauri-cli
```

### Port 5000 already in use
Kill the previous node process:
```powershell
netstat -ano | findstr 5000
taskkill /PID <PID> /F
```

### Node not connecting
Ensure the Freedom Network node is running:
```powershell
cd ../node
cargo run
```

## Future Enhancements

- [ ] Real DHT queries from the browser
- [ ] Encrypted communication between tabs and node
- [ ] Bookmarks and history
- [ ] Settings window
- [ ] Update checker
- [ ] Tor integration layer

## License

Same as Freedom Network

---

**Freedom Browser** - Browse the decentralized internet
