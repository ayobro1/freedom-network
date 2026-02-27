/// HTTP/SOCKS5 Proxy server for routing traffic through Freedom Network
/// This allows standard browsers to use the network via proxy configuration

use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt, copy_bidirectional};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::onion::OnionRouter;
use anyhow::{Result, anyhow};

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

        let session_result = async {
            let mut buffer = vec![0u8; 8192];
            let n = socket.read(&mut buffer).await?;
            if n == 0 {
                return Ok(());
            }

            let mut recv = metrics.bytes_received.write().await;
            *recv += n as u64;
            drop(recv);

            let request = String::from_utf8_lossy(&buffer[..n]).to_string();
            let first_line = request.lines().next().unwrap_or("");
            let parts: Vec<&str> = first_line.split_whitespace().collect();
            if parts.len() < 3 {
                return Err(anyhow!("Invalid HTTP request line"));
            }

            let method = parts[0];
            let path = parts[1];
            println!("📍 {} {}", method, path);

            if method.eq_ignore_ascii_case("CONNECT") {
                let target = Self::normalize_connect_target(path);
                let mut upstream = TcpStream::connect(&target).await?;

                let response = b"HTTP/1.1 200 Connection Established\r\n\r\n";
                socket.write_all(response).await?;

                let mut sent = metrics.bytes_sent.write().await;
                *sent += response.len() as u64;
                drop(sent);

                println!("🔐 CONNECT tunnel established to {}", target);
                let (client_to_upstream, upstream_to_client) = copy_bidirectional(&mut socket, &mut upstream).await?;

                let mut recv = metrics.bytes_received.write().await;
                *recv += client_to_upstream;
                drop(recv);

                let mut sent = metrics.bytes_sent.write().await;
                *sent += upstream_to_client;
                drop(sent);
            } else {
                let target = Self::extract_http_target(path, &request)?;
                let rewritten = Self::rewrite_request_line(&request)?;

                let mut upstream = TcpStream::connect(&target).await?;
                upstream.write_all(rewritten.as_bytes()).await?;

                let (client_to_upstream, upstream_to_client) = copy_bidirectional(&mut socket, &mut upstream).await?;

                let mut recv = metrics.bytes_received.write().await;
                *recv += client_to_upstream;
                drop(recv);

                let mut sent = metrics.bytes_sent.write().await;
                *sent += upstream_to_client;
                drop(sent);

                println!("✓ Forwarded HTTP request through {}", target);
            }

            Ok(())
        }
        .await;

        let mut active = metrics.active_connections.write().await;
        *active = active.saturating_sub(1);
        drop(active);

        session_result
    }

    fn normalize_connect_target(path: &str) -> String {
        if path.contains(':') {
            path.to_string()
        } else {
            format!("{}:443", path)
        }
    }

    fn extract_http_target(path: &str, request: &str) -> Result<String> {
        if let Some(stripped) = path.strip_prefix("http://") {
            let host_port = stripped.split('/').next().unwrap_or("");
            if host_port.is_empty() {
                return Err(anyhow!("Missing host in absolute URL"));
            }
            if host_port.contains(':') {
                return Ok(host_port.to_string());
            }
            return Ok(format!("{}:80", host_port));
        }

        if let Some(stripped) = path.strip_prefix("https://") {
            let host_port = stripped.split('/').next().unwrap_or("");
            if host_port.is_empty() {
                return Err(anyhow!("Missing host in absolute URL"));
            }
            if host_port.contains(':') {
                return Ok(host_port.to_string());
            }
            return Ok(format!("{}:443", host_port));
        }

        for line in request.lines() {
            if line.to_ascii_lowercase().starts_with("host:") {
                let host = line[5..].trim();
                if host.is_empty() {
                    return Err(anyhow!("Host header is empty"));
                }
                if host.contains(':') {
                    return Ok(host.to_string());
                }
                return Ok(format!("{}:80", host));
            }
        }

        Err(anyhow!("Missing Host header for HTTP request"))
    }

    fn rewrite_request_line(request: &str) -> Result<String> {
        let mut lines = request.splitn(2, "\r\n");
        let first_line = lines.next().ok_or_else(|| anyhow!("Missing request line"))?;
        let remainder = lines.next().unwrap_or("");

        let parts: Vec<&str> = first_line.split_whitespace().collect();
        if parts.len() < 3 {
            return Err(anyhow!("Invalid request line"));
        }

        let method = parts[0];
        let path = parts[1];
        let version = parts[2];

        let new_path = if let Some(stripped) = path.strip_prefix("http://") {
            let suffix = stripped.split_once('/').map(|(_, tail)| tail).unwrap_or("");
            if suffix.is_empty() { "/".to_string() } else { format!("/{}", suffix) }
        } else if let Some(stripped) = path.strip_prefix("https://") {
            let suffix = stripped.split_once('/').map(|(_, tail)| tail).unwrap_or("");
            if suffix.is_empty() { "/".to_string() } else { format!("/{}", suffix) }
        } else {
            path.to_string()
        };

        Ok(format!("{} {} {}\r\n{}", method, new_path, version, remainder))
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
