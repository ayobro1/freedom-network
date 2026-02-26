/// Simple web dashboard server for Freedom Network
/// Serves a basic management UI on port 9090

use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use anyhow::Result;

pub struct WebDashboard {
    listener: TcpListener,
}

impl WebDashboard {
    pub async fn new(addr: SocketAddr) -> Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        println!("🖥️  Web Dashboard available at http://{}", addr);
        println!("   View stats, manage VPN, configure proxy\n");
        
        Ok(WebDashboard { listener })
    }

    pub async fn run(&self) -> Result<()> {
        loop {
            let (socket, addr) = self.listener.accept().await?;
            println!("🖥️  Dashboard connection from {}", addr);
            
            tokio::spawn(async move {
                if let Err(e) = Self::handle_request(socket).await {
                    eprintln!("❌ Dashboard error: {}", e);
                }
            });
        }
    }

    async fn handle_request(mut socket: TcpStream) -> Result<()> {
        let mut buffer = vec![0u8; 4096];
        let n = socket.read(&mut buffer).await?;
        
        if n == 0 {
            return Ok(());
        }

        let request = String::from_utf8_lossy(&buffer[..n]);
        let lines: Vec<&str> = request.lines().collect();
        
        if lines.is_empty() {
            return Ok(());
        }

        let request_line = lines[0].split_whitespace().collect::<Vec<_>>();
        if request_line.len() < 2 {
            return Ok(());
        }

        let method = request_line[0];
        let path = request_line[1];

        // Route requests
        let response = match (method, path) {
            ("GET", "/") => Self::html_dashboard(),
            ("GET", "/api/status") => Self::api_status(),
            ("GET", "/api/config") => Self::api_config(),
            ("GET", "/api/stats") => Self::api_stats(),
            _ => Self::not_found(),
        };

        socket.write_all(response.as_bytes()).await?;
        Ok(())
    }

    fn html_dashboard() -> String {
        let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Freedom Network - VPN Dashboard</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: #0a0e27; color: #e5e7eb; }
        .container { max-width: 1200px; margin: 0 auto; padding: 40px 20px; }
        header { border-bottom: 1px solid #2d3748; padding-bottom: 30px; margin-bottom: 40px; }
        h1 { font-size: 2em; margin-bottom: 5px; background: linear-gradient(135deg, #4a9eff, #7c3aed); -webkit-background-clip: text; -webkit-text-fill-color: transparent; }
        .status { display: flex; align-items: center; gap: 8px; padding: 8px 16px; background: #151b35; border: 1px solid #10b981; border-radius: 20px; width: fit-content; margin-top: 15px; }
        .dot { width: 8px; height: 8px; background: #10b981; border-radius: 50%; animation: pulse 2s infinite; }
        @keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.5; } }
        .cards { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; margin-bottom: 40px; }
        .card { background: #151b35; border: 1px solid #2d3748; border-radius: 8px; overflow: hidden; }
        .card-header { padding: 16px 20px; background: #1e2749; font-weight: 600; border-bottom: 1px solid #2d3748; }
        .card-content { padding: 20px; }
        .stat { display: flex; justify-content: space-between; padding: 10px 0; border-bottom: 1px solid #2d3748; }
        .stat:last-child { border: none; }
        .label { color: #a0aec0; }
        .value { font-weight: 600; color: #4a9eff; }
        code { background: #1e2749; padding: 2px 6px; border-radius: 3px; font-family: monospace; }
        h2 { font-size: 1.3em; margin-bottom: 20px; }
        .setup { background: #151b35; border: 1px solid #2d3748; border-radius: 8px; padding: 30px; margin-bottom: 40px; }
        ol { margin-left: 20px; line-height: 1.8; }
        li { margin-bottom: 10px; }
        footer { text-align: center; padding: 40px 0; border-top: 1px solid #2d3748; color: #a0aec0; }
        a { color: #4a9eff; text-decoration: none; }
        a:hover { text-decoration: underline; }
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>🌐 Freedom Network</h1>
            <p>Decentralized VPN & Proxy Service</p>
            <div class="status">
                <span class="dot"></span> Running
            </div>
        </header>

        <div class="cards">
            <div class="card">
                <div class="card-header">📍 Network Status</div>
                <div class="card-content">
                    <div class="stat"><span class="label">Status:</span> <span class="value">Running</span></div>
                    <div class="stat"><span class="label">Node ID:</span> <span class="value">f4e2a1b3</span></div>
                    <div class="stat"><span class="label">Connected Peers:</span> <span class="value" id="peers">—</span></div>
                    <div class="stat"><span class="label">Active Circuits:</span> <span class="value" id="circuits">—</span></div>
                </div>
            </div>

            <div class="card">
                <div class="card-header">🔐 Proxy Configuration</div>
                <div class="card-content">
                    <div class="stat"><span class="label">HTTP Proxy:</span> <span class="value"><code>127.0.0.1:8080</code></span></div>
                    <div class="stat"><span class="label">QUIC Server:</span> <span class="value"><code>127.0.0.1:5000</code></span></div>
                    <div class="stat"><span class="label">Encryption:</span> <span class="value">ChaCha20-Poly1305</span></div>
                </div>
            </div>

            <div class="card">
                <div class="card-header">📊 Data Transfer</div>
                <div class="card-content">
                    <div class="stat"><span class="label">Sent:</span> <span class="value" id="sent">—</span></div>
                    <div class="stat"><span class="label">Received:</span> <span class="value" id="recv">—</span></div>
                    <div class="stat"><span class="label">Connections:</span> <span class="value" id="conns">—</span></div>
                </div>
            </div>
        </div>

        <div class="setup">
            <h2>🔧 Configure Your Browser</h2>
            <p><strong>Firefox:</strong> Settings → Network Settings → Manual proxy → HTTP: 127.0.0.1, Port: 8080</p>
            <br>
            <p><strong>Chrome/Edge:</strong> Settings → Advanced → System → Open proxy settings → Manual proxy → HTTP: 127.0.0.1:8080</p>
            <br>
            <p><strong>Safari:</strong> System Preferences → Network → Advanced → Proxies → Web Proxy (HTTP) → 127.0.0.1:8080</p>
        </div>

        <footer>
            <p>Freedom Network v1.0 | GNU AGPLv3 | <a href="https://github.com/ayobro1/freedom-network" target="_blank">GitHub</a></p>
        </footer>
    </div>

    <script>
        async function updateStatus() {
            try {
                const resp = await fetch('/api/status');
                const data = await resp.json();
                document.getElementById('peers').textContent = data.peers_connected;
                document.getElementById('circuits').textContent = data.active_circuits;
            } catch (e) { console.error(e); }
        }
        async function updateStats() {
            try {
                const resp = await fetch('/api/stats');
                const data = await resp.json();
                document.getElementById('sent').textContent = Math.round(data.bytes_sent / 1024) + ' KB';
                document.getElementById('recv').textContent = Math.round(data.bytes_received / 1024) + ' KB';
                document.getElementById('conns').textContent = data.connections_active + '/' + data.connections_total;
            } catch (e) { console.error(e); }
        }
        setInterval(updateStatus, 2000);
        setInterval(updateStats, 2000);
        updateStatus();
        updateStats();
    </script>
</body>
</html>"#;

        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
            html.len(),
            html
        )
    }

    fn api_status() -> String {
        let json = r#"{"status":"running","quic_server":"127.0.0.1:5000","http_proxy":"127.0.0.1:8080","uptime_seconds":120,"peers_connected":5,"active_circuits":3,"node_id":"f4e2a1b3c5d7e9f1","version":"1.0.0"}"#;
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            json.len(),
            json
        )
    }

    fn api_config() -> String {
        let json = r#"{"proxy_enabled":true,"proxy_address":"127.0.0.1:8080","quic_address":"127.0.0.1:5000","dashboard_address":"127.0.0.1:9090","dht_enabled":true,"onion_routing":true,"max_circuits":10,"circuit_hops":3}"#;
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            json.len(),
            json
        )
    }

    fn api_stats() -> String {
        let json = r#"{"bytes_sent":1024000,"bytes_received":2048000,"packets_sent":5120,"packets_received":10240,"connections_total":8,"connections_active":3,"dht_lookups":12,"circuit_builds":5,"circuit_failures":1}"#;
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            json.len(),
            json
        )
    }

    fn not_found() -> String {
        format!("HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n")
    }
}
