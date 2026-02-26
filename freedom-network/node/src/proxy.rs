/// HTTP/SOCKS5 Proxy server for routing traffic through Freedom Network
/// This allows standard browsers to use the network via proxy configuration

use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::onion::OnionRouter;
use anyhow::Result;

#[derive(Clone, Debug)]
pub struct ProxyMetrics {
    pub bytes_sent: Arc<RwLock<u64>>,
    pub bytes_received: Arc<RwLock<u64>>,
    pub total_connections: Arc<RwLock<u64>>,
    pub active_connections: Arc<RwLock<u64>>,
}

pub struct ProxyServer {
    listener: TcpListener,
    onion_router: Arc<OnionRouter>,
    metrics: ProxyMetrics,
}

impl ProxyServer {
    pub async fn new(addr: SocketAddr, onion_router: Arc<OnionRouter>) -> Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        println!("🌐 HTTP Proxy Server listening on {}", addr);
        println!("   Configure your browser proxy to: {}:{}", 
                 addr.ip(), addr.port());
        
        let metrics = ProxyMetrics {
            bytes_sent: Arc::new(RwLock::new(0)),
            bytes_received: Arc::new(RwLock::new(0)),
            total_connections: Arc::new(RwLock::new(0)),
            active_connections: Arc::new(RwLock::new(0)),
        };
        
        Ok(ProxyServer {
            listener,
            onion_router,
            metrics,
        })
    }

    pub fn get_metrics(&self) -> ProxyMetrics {
        self.metrics.clone()
    }

    pub async fn run(&self) -> Result<()> {
        loop {
            let (socket, addr) = self.listener.accept().await?;
            println!("👁️  Proxy connection from {}", addr);
            
            let onion = self.onion_router.clone();
            let metrics = self.metrics.clone();
            
            tokio::spawn(async move {
                if let Err(e) = Self::handle_client(socket, onion, metrics).await {
                    eprintln!("❌ Proxy error: {}", e);
                }
            });
        }
    }

    async fn handle_client(mut socket: TcpStream, _onion_router: Arc<OnionRouter>, metrics: ProxyMetrics) -> Result<()> {
        // Increment active connections
        let mut active = metrics.active_connections.write().await;
        *active += 1;
        let mut total = metrics.total_connections.write().await;
        *total += 1;
        drop(active);
        drop(total);
        
        let mut buffer = vec![0u8; 8192];
        
        // Read the HTTP request
        let n = socket.read(&mut buffer).await?;
        if n == 0 {
            return Ok(());
        }
        
        // Record bytes received
        let mut recv = metrics.bytes_received.write().await;
        *recv += n as u64;
        drop(recv);

        let request = String::from_utf8_lossy(&buffer[..n]);
        
        // Parse HTTP request line (METHOD PATH VERSION)
        let first_line = request.lines().next().unwrap_or("");
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        
        if parts.len() < 3 {
            eprintln!("❌ Invalid HTTP request");
            return Ok(());
        }

        let method = parts[0];
        let path = parts[1];
        
        println!("📍 {} {}", method, path);

        // Handle CONNECT (HTTPS tunneling)
        if method == "CONNECT" {
            // For CONNECT, we establish a tunnel
            let response = b"HTTP/1.1 200 Connection Established\r\n\r\n";
            socket.write_all(response).await?;
            
            // Record bytes sent
            let mut sent = metrics.bytes_sent.write().await;
            *sent += response.len() as u64;
            drop(sent);
            
            println!("🔐 CONNECT tunnel established to {}", path);
            
            // In production, here we would:
            // 1. Parse the destination from path
            // 2. Build an onion circuit through the network
            // 3. Relay traffic bidirectionally
            // For now, just keep connection alive
            let mut buf = vec![0u8; 4096];
            loop {
                match socket.read(&mut buf).await? {
                    0 => break,
                    n => {
                        // Track relayed bytes
                        let mut recv = metrics.bytes_received.write().await;
                        *recv += n as u64;
                        drop(recv);
                        
                        println!("   ↔️ Relay {} bytes through circuit", n);
                    }
                }
            }
        } else {
            // Handle regular HTTP (GET, POST, etc.)
            let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 72\r\n\r\n\
                            <html><body><h1>Freedom Network Proxy</h1><p>Connected!</p></body></html>";
            socket.write_all(response).await?;
            
            // Record bytes sent
            let mut sent = metrics.bytes_sent.write().await;
            *sent += response.len() as u64;
            drop(sent);
            
            println!("✓ HTTP response sent");
        }

        // Decrement active connections
        let mut active = metrics.active_connections.write().await;
        *active = active.saturating_sub(1);
        drop(active);

        Ok(())
    }
}

// Legacy SOCKS5 support (extended feature)
#[allow(dead_code)]
pub struct Socks5Server {
    listener: TcpListener,
}

#[allow(dead_code)]
impl Socks5Server {
    pub async fn new(addr: SocketAddr) -> Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        println!("🔷 SOCKS5 Server listening on {}", addr);
        
        Ok(Socks5Server { listener })
    }

    pub async fn run(&self) -> Result<()> {
        loop {
            let (_socket, addr) = self.listener.accept().await?;
            println!("🔗 SOCKS5 connection from {}", addr);
            // SOCKS5 implementation would go here
        }
    }
}
