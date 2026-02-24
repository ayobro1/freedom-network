# Freedom Network - Tor-Like Decentralized Network

A peer-to-peer network built on QUIC protocol with `.freedom` domains, multi-hop routing, DHT-based peer discovery, and decentralized content distribution.

## Architecture Overview

### Core Components

#### 1. **DHT (Distributed Hash Table)** - `protocol.rs`
- Kademlia-based peer discovery
- `.freedom` domain registration and lookup
- Peer address resolution
- Content metadata storage

**Key Functions:**
- `register_domain()` - Register a `.freedom` site
- `lookup_domain()` - Find site owner and metadata
- `xor_distance()` - Calculate peer distance for clustering
- `find_closest_peers()` - Find peers by proximity

#### 2. **Multi-Hop Routing** - `routing.rs`
- Circuit-based routing (like Tor)
- Build multi-hop circuits through relay nodes
- Support for encrypted data relay
- Circuit lifecycle management

**Key Structure:**
- `Circuit` - Path through network hops
- `CircuitId` - Unique circuit identifier
- `Router` - Manages active circuits

#### 3. **Site Server** - `sites.rs`
- Host `.freedom` sites locally
- Serve `.fdom` content files
- Directory-based site management
- Path traversal protection

**Key Functions:**
- `register_site()` - Add a site to host
- `serve_file()` - Retrieve site content
- `get_site_info()` - Get site metadata

#### 4. **Resolver** - `resolver.rs`
- Resolve `.freedom` domains to network addresses
- DHT lookups with caching
- Bootstrap node connectivity
- Metadata caching

#### 5. **Network Client** - `client.rs`
- Used by browser to fetch from sites
- Handle `.freedom` requests
- Connect to exit nodes
- Manage HTTP-like requests over P2P network

#### 6. **Node Server** - `main.rs`
- Run QUIC server
- Accept incoming connections
- Handle DHT messages
- Manage registered domains

## Network Protocol

### Message Types

#### DHT Messages
```rust
FindFreedomDomain { domain: String }
StoreFreedomDomain { domain, owner_node, pubkey }
FindNode { target: NodeId, requesting_node: NodeId }
```

#### Routing Messages
```rust
BuildCircuit { hops: Vec<NodeId>, circuit_id: u32 }
RelayData { circuit_id: u32, data: Vec<u8> }
DestroyCircuit { circuit_id: u32 }
```

#### Content Messages
```rust
GetContent { domain, path, circuit_id }
ContentData { data, metadata }
NotFound
```

## .freedom Domain System

### Domain Format
- `.freedom` TLD for decentralized sites
- Example: `example.freedom`
- Each domain has:
  - Owner node ID
  - Cryptographic public key
  - Node address (IP:port)
  - Content metadata

### Domain Registration

1. **Node generates cryptographic identity**
2. **Publishes domain via DHT**
   - DHT registers mapping: `domain.freedom` → `NodeId`
   - Peers learn about new domains
3. **Other nodes cache the mapping**
4. **Lookups query DHT with caching**

## Multi-Hop Routing (Onion Routing)

### Circuit Building
1. Select relay nodes
2. Create circuit through them
3. Each hop encrypts for next hop
4. Exit node handles content request

### Path Example
```
Browser → Node1 → Node2 → Node3 (Exit)
           ↓      ↓      ↓
        Relay1  Relay2  Content Server
```

### Security Properties
- Entry node doesn't know exit
- Exit node doesn't know origin
- Intermediate nodes don't know content
- Each connection encrypted separately

## Content Distribution

### Site Hosting
1. **Create site folder with `.fdom` files**
2. **Register with local node:**
   ```
   node.register_site("mysite.freedom", "/path/to/content", "index.html")
   ```
3. **Node publishes to DHT**
4. **Other peers query DHT to find you**

### Content Fetching
1. **Browser requests `mysite.freedom/page.html`**
2. **Resolver queries DHT**
3. **Build circuit to exit node at site owner**
4. **Request routed through circuit**
5. **Content returns encrypted through circuit**

## Running the Network

### Standard Node
```bash
./start-node.bat
```

Listens on `127.0.0.1:5000`
- Registers `node.freedom`
- Accepts connections from other nodes
- Participates in DHT

### Browser
```bash
# Opens web UI at http://localhost
./start-browser-ui.bat
```

Features:
- Click tabs to switch sites
- Type `freedom://` addresses
- Real P2P fetching through network
- Multi-hop routing support

## File Structure

```
freedom-network/
├── node/                    # QUIC node server
│   ├── src/
│   │   ├── main.rs        # Node server & DHT
│   │   ├── protocol.rs    # DHT & messages
│   │   ├── routing.rs     # Circuit building
│   │   ├── sites.rs       # Content hosting
│   │   ├── resolver.rs    # Domain resolution
│   │   ├── client.rs      # Network client
│   │   └── ...
│   └── Cargo.toml
├── browser/                 # Rust GUI (can be replaced)
├── ui/                      # Web UI
│   ├── index.html
│   ├── style.css
│   └── app.js
└── scripts/
    ├── start-node.bat
    ├── start-browser-ui.bat
    └── ...
```

## Security Model

### Cryptography
- **TLS 1.3** for transport (via QUIC)
- **Ed25519** for node identity
- **ChaCha20-Poly1305** for content encryption
- **SHA3-256** for node IDs and content hashing

### Privacy
- Multi-hop routing prevents correlation
- No single node sees full path
- Exit node doesn't know requester
- Content encrypted end-to-end

## Future Enhancements

1. **Real .freedom TLD** - Integrate with network stack
2. **Onion Routing Encryption** - Full end-to-end encryption
3. **Content Pinning** - Distribute popular content across peers
4. **Bandwidth Sharing** - Incentivize relay participation
5. **Hidden Services** - Run services without exposing identity
6. **Cross-platform** - Desktop and mobile clients
7. **Exit Nodes** - Deploy public exit infrastructure

## Testing

```bash
# Build all components
./scripts/setup.bat

# Run node
./scripts/start-node.bat

# Open browser UI
./scripts/start-browser-ui.bat
```

### Test Scenario
1. Start node → registers `node.freedom`
2. Open browser → connects to local node
3. Click tabs or type addresses → fetches content through DHT

## Design Inspiration

- **Tor** - Multi-hop routing, circuit switching
- **IPFS** - DHT, decentralized content
- **Bitcoin** - P2P network, peer discovery
- **OnionShare** - Decentralized file sharing
- **.onion domains** - Cryptographic addressing

## Protocol Differences from Tor

| Feature | Freedom Network | Tor |
|---------|-----------------|-----|
| Transport | QUIC | TCP |
| Naming | .freedom (DHT) | .onion (hidden service) |
| Content | Native support | Exit services |
| Routing | Circuit-based | Stream-based |
| Target | General web | Anonymity focus |

---

**Built with Rust, QUIC, and peer-to-peer networking.**
