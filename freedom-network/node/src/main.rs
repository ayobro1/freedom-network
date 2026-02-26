mod protocol;
mod routing;
mod encrypt;
mod identity;
mod utils;
mod sites;
mod resolver;
mod client;
mod onion;
mod proxy;
mod web;

use std::sync::Arc;
use quinn::{Endpoint, ServerConfig};
use rcgen::generate_simple_self_signed;
use std::net::SocketAddr;
use protocol::{DHT, NodeId, FreedomAddress};
use routing::Router;
use std::collections::HashMap;
use tokio::sync::RwLock;
use sha3::Digest;
use proxy::ProxyServer;
use web::WebDashboard;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Force stdout to flush immediately
    use std::io::{self, Write};
    let _ = io::stdout().flush();
    
    eprintln!("🌐 Starting Freedom Network Node...");
    println!("🌐 Freedom Network Node");
    println!("========================\n");
    let _ = io::stdout().flush();

    // Initialize node infrastructure
    let dht = Arc::new(DHT::new());
    let router = Arc::new(Router::new());
    let onion_router = Arc::new(onion::OnionRouter::new());
    let _domain_cache: Arc<RwLock<HashMap<String, String>>> = Arc::new(RwLock::new(HashMap::new()));
    // Generate node identity
    let cert = generate_simple_self_signed(vec!["localhost".into()])?;
    let cert_der = cert.serialize_der()?;
    let key_der = cert.serialize_private_key_der();

    // Create node ID from certificate
    let mut hasher = sha3::Sha3_256::new();
    hasher.update(&cert_der);
    let node_id_hash = hasher.finalize();
    let mut node_id_bytes = [0u8; 32];
    node_id_bytes.copy_from_slice(&node_id_hash[..32]);
    let node_id = NodeId(node_id_bytes);

    println!("📍 Node ID: {}", hex::encode(&node_id.0[..8]));

    // Initialize onion routing
    println!("🧅 Onion Routing Layer: Initialized");
    println!("   - Multi-hop circuit support");
    println!("   - Layer encryption (Tor-like)");
    println!("   - Privacy-preserving routing\n");

    // Set up QUIC server
    let mut server_config = ServerConfig::with_single_cert(
        vec![rustls::Certificate(cert_der.clone())],
        rustls::PrivateKey(key_der)
    )?;
    server_config.transport = Arc::new(quinn::TransportConfig::default());

    let addr: SocketAddr = "127.0.0.1:5000".parse()?;
    let endpoint = Endpoint::server(server_config, addr)?;
    println!("🚀 QUIC Server listening on {}", addr);
    println!("🔐 TLS Certificate: {} bytes\n", cert_der.len());

    // Register this node in the DHT
    println!("📝 Registering node in DHT...");
    let freedom_address = FreedomAddress {
        domain: "node.freedom".to_string(),
        node_id: node_id.clone(),
        ed25519_pubkey: cert_der.clone(),
    };
    dht.register_domain(freedom_address.clone());
    println!("✓ Registered: {}\n", freedom_address.domain);

    // Initialize HTTP Proxy Server (VPN-like interface)
    let proxy_addr: SocketAddr = "127.0.0.1:8080".parse()?;
    let proxy_server = Arc::new(ProxyServer::new(proxy_addr, onion_router.clone()).await?);
    
    println!("\n╔════════════════════════════════════════════╗");
    println!("║     FREEDOM NETWORK VPN PROXY ACTIVE      ║");
    println!("╠════════════════════════════════════════════╣");
    println!("║ 📍 Proxy: http://127.0.0.1:8080           ║");
    println!("║ 🌐 Configure your browser:                 ║");
    println!("║    Firefox: Preferences → Network Settings ║");
    println!("║    Chrome: Settings → Advanced → Proxy     ║");
    println!("║    Set HTTP proxy to: 127.0.0.1:8080       ║");
    println!("╚════════════════════════════════════════════╝\n");

    // Spawn proxy server task
    let proxy_clone = proxy_server.clone();
    tokio::spawn(async move {
        if let Err(e) = proxy_clone.run().await {
            eprintln!("🔴 Proxy server error: {}", e);
        }
    });

    // Initialize Web Dashboard with proxy metrics
    let proxy_metrics = proxy_server.get_metrics();
    let dashboard_addr: SocketAddr = "127.0.0.1:9090".parse()?;
    let web_dashboard = Arc::new(WebDashboard::new(dashboard_addr, proxy_metrics).await?);
    
    println!("🖥️  Dashboard: http://127.0.0.1:9090\n");

    // Spawn web dashboard task
    let web_clone = web_dashboard.clone();
    tokio::spawn(async move {
        if let Err(e) = web_clone.run().await {
            eprintln!("🔴 Web dashboard error: {}", e);
        }
    });

    // Main loop: accept incoming QUIC connections
    println!("⏳ Waiting for connections...\n");
    loop {
        if let Some(conn) = endpoint.accept().await {
            let _dht = dht.clone();
            let _router = router.clone();

            tokio::spawn(async move {
                if let Ok(new_conn) = conn.await {
                    println!("🔗 New connection from {}", new_conn.remote_address());

                    loop {
                        match new_conn.accept_bi().await {
                            Ok((mut send, mut recv)) => {
                                let mut buf = vec![0; 8192];
                                match recv.read(&mut buf).await {
                                    Ok(Some(n)) => {
                                        println!("📨 Received {} bytes", n);
                                        let response = b"ACK";
                                        let _ = send.write_all(response).await;
                                    }
                                    Ok(None) => break,
                                    Err(e) => {
                                        eprintln!("❌ Read error: {}", e);
                                        break;
                                    }
                                }
                            }
                            Err(_) => break,
                        }
                    }
                    println!("🔌 Connection closed");
                }
            });
        }
    }
}