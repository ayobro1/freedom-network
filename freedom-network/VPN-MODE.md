# Freedom Network - VPN/Proxy Mode

Freedom Network now runs as a local VPN/proxy service that works with any standard web browser (Chrome, Firefox, Safari, Edge, etc.).

## Quick Start

### 1. **Start the VPN Service**

Simply double-click `start-vpn.bat` (on Windows) or run:

```bash
cd node
cargo build --release
./target/release/freedom-node
```

The service will start and listen on: **http://127.0.0.1:8080**

### 2. **Configure Your Browser**

#### Firefox
1. Open **Preferences** → **Network Settings**
2. Scroll down to "Configure Proxy Access to the Internet"
3. Select **Manual proxy configuration**
4. Set **HTTP Proxy:** `127.0.0.1` and **Port:** `8080`
5. Done! All traffic is now routed through Freedom Network

#### Chrome / Edge
1. Open **Settings** → **Advanced** → **System**
2. Click **Open your computer's proxy settings**
3. Enable **Manual proxy setup**
4. Set **HTTP Proxy:** `127.0.0.1` and **Port:** `8080`
5. Done!

#### Safari (macOS)
1. **System Preferences** → **Network**
2. Click **Advanced...** → **Proxies**
3. Check **Web Proxy (HTTP)**
4. Proxy Server: `127.0.0.1`, Port: `8080`

### 3. **Browse Freely**

Once configured, all your browser traffic is routed through the Freedom Network:

- **Decentralized routing** via Kademlia DHT
- **Multi-hop circuits** (Tor-like onion routing)
- **No central server** - fully peer-to-peer
- **End-to-end encryption** via TLS 1.3 and ChaCha20-Poly1305

## How It Works

```
Your Browser
    ↓
    └─→ HTTP CONNECT to 127.0.0.1:8080 (Freedom Network Proxy)
            ↓
            └─→ Builds onion circuit through DHT peers
                    ↓
                    └─→ Routes through multi-hop encrypted tunnels
                            ↓
                            └─→ Exits to destination (all encrypted)
                                    ↓
                                    └─→ Response travels back through circuit
                                            ↓
                                            └─→ Browser receives data
```

### Proxy Details

- **Proxy Type:** HTTP CONNECT (supports HTTPS)
- **Address:** `127.0.0.1:8080`
- **Protocol:** HTTP/1.1
- **Encryption:** Routes through Freedom Network's TLS + ChaCha20
- **Routing:** Multi-hop Tor-like circuits with symmetric key encryption
- **Node Discovery:** Kademlia DHT peer lookup

## Network Architecture

The node performs:

1. **DHT (Kademlia)** - Distributed peer discovery
2. **Onion Routing** - Multi-hop private circuits
3. **HTTP Proxy** - Browser-compatible interface
4. **TLS/QUIC** - Secure transport layer
5. **Content Addressing** - Freedom domain registry

## Stopping the Service

Press **Ctrl+C** in the terminal window to stop the VPN service.

## Troubleshooting

### "Connection refused" when browsing
- Make sure the VPN service is running (check for "QUIC Server listening on 127.0.0.1:5000")
- Verify browser proxy is set to `127.0.0.1:8080`
- Check Windows Firewall isn't blocking port 8080

### Slow connections
- Network latency varies based on DHT peer distance
- Multiple hops add latency for privacy
- First connection to a domain may take longer (DNS-like lookup)

### Can't access a website
- The exit node in your circuit must reach the destination
- Multi-hop circuit might not have connectivity to some sites (by design)
- Clear browser cache and retry

## Security Notes

🔐 **Privacy-First Design:**
- No logs stored by default
- Each circuit uses unique encryption keys
- Layer-by-layer encryption (like Tor)
- Your ISP cannot see destination URLs
- Recipient cannot see your IP address

⚠️ **Limitations:**
- This is an early-stage research prototype
- Not suitable for production security-critical use yet
- Onion routing circuits may be slow (privacy tradeoff)
- Some techniques are theoretical implementations

## Building from Source

```bash
cd freedom-network/node
cargo build --release
./target/release/freedom-node
```

## Advanced Usage

### Command-line options (coming soon)
```bash
freedom-node --proxy 127.0.0.1:9090  # Custom proxy port
freedom-node --socks5                 # Enable SOCKS5 (if available)
freedom-node --config config.toml     # Custom config file
```

## Contributing

The Freedom Network is open-source and welcomes contributions:

- **Protocol improvements** - Better routing algorithms
- **Performance** - Circuit caching, connection pooling
- **UI/UX** - Graphical menu, status widget
- **Testing** - Multi-node network tests
- **Documentation** - User guides, technical specs

See `ARCHITECTURE.md` for technical details.

---

**Status:** Beta - Research Prototype  
**License:** GNU AGPLv3  
**Repository:** https://github.com/ayobro1/freedom-network
