/// Web dashboard server for Freedom Network management
/// Serves real-time statistics and configuration UI

use std::net::SocketAddr;
use std::time::SystemTime;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use anyhow::Result;
use crate::proxy::ProxyMetrics;

pub struct WebDashboard {
    listener: TcpListener,
    proxy_metrics: ProxyMetrics,
    start_time: SystemTime,
}

impl WebDashboard {
    pub async fn new(addr: SocketAddr, proxy_metrics: ProxyMetrics) -> Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        println!("🖥️  Web Dashboard available at http://{}", addr);
        println!("   View stats, manage VPN, configure proxy\n");
        
        Ok(WebDashboard { 
            listener,
            proxy_metrics,
            start_time: SystemTime::now(),
        })
    }

    pub async fn run(&self) -> Result<()> {
        loop {
            let (socket, addr) = self.listener.accept().await?;
            println!("🖥️  Dashboard connection from {}", addr);
            
            let proxy_metrics = self.proxy_metrics.clone();
            let start_time = self.start_time;
            
            tokio::spawn(async move {
                if let Err(e) = Self::handle_request(socket, proxy_metrics, start_time).await {
                    eprintln!("❌ Dashboard error: {}", e);
                }
            });
        }
    }

    async fn handle_request(
        mut socket: TcpStream,
        proxy_metrics: ProxyMetrics,
        start_time: SystemTime,
    ) -> Result<()> {
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
            ("GET", "/") => Self::html_dashboard(&proxy_metrics, start_time).await,
            ("GET", "/api/status") => Self::api_status(&proxy_metrics, start_time).await,
            ("GET", "/api/config") => Self::api_config().await,
            ("GET", "/api/stats") => Self::api_stats(&proxy_metrics).await,
            ("OPTIONS", _) => "HTTP/1.1 204 No Content\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\nContent-Length: 0\r\n\r\n".to_string(),
            _ => Self::not_found().await,
        };

        socket.write_all(response.as_bytes()).await?;
        Ok(())
    }

    async fn html_dashboard(_proxy_metrics: &ProxyMetrics, _start_time: SystemTime) -> String {
        let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Freedom Network — Dashboard</title>
    <style>
        *, *::before, *::after { margin: 0; padding: 0; box-sizing: border-box; }
        :root {
            --bg: #0a0e27; --bg2: #151b3d; --bg3: #1e2749;
            --text: #e0e6ff; --muted: #a0a6c3;
            --blue: #4a9eff; --purple: #7c3aed; --green: #10b981;
            --border: #2d3547;
        }
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: var(--bg); color: var(--text); min-height: 100vh; }
        a { color: var(--blue); text-decoration: none; }
        a:hover { text-decoration: underline; }
        code { background: var(--bg3); padding: 2px 7px; border-radius: 4px; font-family: 'Courier New', monospace; color: var(--blue); font-size: 13px; }

        /* Layout */
        .shell { max-width: 1100px; margin: 0 auto; padding: 36px 24px; }

        /* Header */
        .header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 36px; flex-wrap: wrap; gap: 16px; }
        .brand { display: flex; align-items: center; gap: 14px; }
        .brand-icon { font-size: 36px; }
        .brand-title { font-size: 1.7em; font-weight: 700; background: linear-gradient(135deg, var(--blue), var(--purple)); -webkit-background-clip: text; -webkit-text-fill-color: transparent; background-clip: text; }
        .brand-sub { font-size: 13px; color: var(--muted); margin-top: 2px; }
        .badge { display: flex; align-items: center; gap: 8px; padding: 8px 18px; background: var(--bg2); border: 1px solid var(--green); border-radius: 20px; font-size: 13px; color: var(--green); }
        .dot { width: 8px; height: 8px; background: var(--green); border-radius: 50%; animation: pulse 2s infinite; }
        @keyframes pulse { 0%, 100% { opacity: 1; box-shadow: 0 0 0 0 rgba(16,185,129,0.4); } 50% { opacity: 0.7; box-shadow: 0 0 0 4px rgba(16,185,129,0); } }

        /* Stat banner */
        .stat-banner { display: grid; grid-template-columns: repeat(auto-fit, minmax(160px, 1fr)); gap: 14px; margin-bottom: 32px; }
        .stat-tile { background: var(--bg2); border: 1px solid var(--border); border-radius: 12px; padding: 18px 16px; display: flex; flex-direction: column; gap: 6px; transition: border-color 0.2s; }
        .stat-tile:hover { border-color: var(--blue); }
        .stat-tile-label { font-size: 11px; color: var(--muted); text-transform: uppercase; letter-spacing: 0.5px; }
        .stat-tile-value { font-size: 22px; font-weight: 700; color: var(--blue); }

        /* Cards */
        .cards { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; margin-bottom: 32px; }
        .card { background: var(--bg2); border: 1px solid var(--border); border-radius: 12px; overflow: hidden; }
        .card-head { padding: 14px 20px; background: var(--bg3); font-weight: 600; font-size: 14px; border-bottom: 1px solid var(--border); display: flex; align-items: center; gap: 8px; }
        .card-body { padding: 20px; }
        .row { display: flex; justify-content: space-between; align-items: center; padding: 10px 0; border-bottom: 1px solid var(--border); font-size: 14px; }
        .row:last-child { border: none; padding-bottom: 0; }
        .row-label { color: var(--muted); }
        .row-value { font-weight: 600; color: var(--blue); }

        /* Setup */
        .setup { background: var(--bg2); border: 1px solid var(--border); border-radius: 12px; padding: 28px; margin-bottom: 32px; }
        .setup h2 { font-size: 15px; font-weight: 700; margin-bottom: 18px; color: var(--blue); text-transform: uppercase; letter-spacing: 0.5px; }
        .steps { display: flex; flex-direction: column; gap: 14px; }
        .step { display: flex; gap: 14px; align-items: flex-start; }
        .step-num { flex-shrink: 0; width: 26px; height: 26px; border-radius: 50%; background: linear-gradient(135deg, var(--blue), var(--purple)); display: flex; align-items: center; justify-content: center; font-size: 12px; font-weight: 700; margin-top: 2px; }
        .step-text { font-size: 14px; color: var(--muted); line-height: 1.5; }
        .step-text strong { color: var(--text); }

        footer { text-align: center; padding: 28px 0 8px; border-top: 1px solid var(--border); color: var(--muted); font-size: 13px; }
    </style>
</head>
<body>
    <div class="shell">
        <div class="header">
            <div class="brand">
                <div class="brand-icon">🌐</div>
                <div>
                    <div class="brand-title">Freedom Network</div>
                    <div class="brand-sub">Decentralized VPN &amp; Proxy</div>
                </div>
            </div>
            <div class="badge"><span class="dot"></span> Running</div>
        </div>

        <div class="stat-banner">
            <div class="stat-tile">
                <div class="stat-tile-label">⏱ Uptime</div>
                <div class="stat-tile-value" id="uptime">—</div>
            </div>
            <div class="stat-tile">
                <div class="stat-tile-label">🔗 Active</div>
                <div class="stat-tile-value" id="connections">—</div>
            </div>
            <div class="stat-tile">
                <div class="stat-tile-label">📊 Total</div>
                <div class="stat-tile-value" id="totalconns">—</div>
            </div>
            <div class="stat-tile">
                <div class="stat-tile-label">⬆ Sent</div>
                <div class="stat-tile-value" id="sent">—</div>
            </div>
            <div class="stat-tile">
                <div class="stat-tile-label">⬇ Received</div>
                <div class="stat-tile-value" id="recv">—</div>
            </div>
            <div class="stat-tile">
                <div class="stat-tile-label">🔀 Total Transfer</div>
                <div class="stat-tile-value" id="total">—</div>
            </div>
        </div>

        <div class="cards">
            <div class="card">
                <div class="card-head">🔐 Endpoints</div>
                <div class="card-body">
                    <div class="row"><span class="row-label">HTTP Proxy</span> <code>127.0.0.1:8080</code></div>
                    <div class="row"><span class="row-label">QUIC Server</span> <code>127.0.0.1:5000</code></div>
                    <div class="row"><span class="row-label">Dashboard</span> <code>127.0.0.1:9090</code></div>
                </div>
            </div>
            <div class="card">
                <div class="card-head">🛡 Security</div>
                <div class="card-body">
                    <div class="row"><span class="row-label">Cipher</span> <span class="row-value">ChaCha20-Poly1305</span></div>
                    <div class="row"><span class="row-label">Transport</span> <span class="row-value">QUIC / TLS 1.3</span></div>
                    <div class="row"><span class="row-label">Routing</span> <span class="row-value">Onion (multi-hop)</span></div>
                    <div class="row"><span class="row-label">Discovery</span> <span class="row-value">DHT</span></div>
                </div>
            </div>
        </div>

        <div class="setup">
            <h2>🔧 Configure Your Browser</h2>
            <div class="steps">
                <div class="step"><div class="step-num">1</div><div class="step-text"><strong>Firefox:</strong> Settings → Network Settings → Manual proxy configuration → HTTP Proxy: <code>127.0.0.1</code>, Port: <code>8080</code></div></div>
                <div class="step"><div class="step-num">2</div><div class="step-text"><strong>Chrome / Edge:</strong> Settings → Advanced → System → Open proxy settings → Manual proxy → <code>127.0.0.1:8080</code></div></div>
                <div class="step"><div class="step-num">3</div><div class="step-text"><strong>Safari / macOS:</strong> System Settings → Network → Advanced → Proxies → Web Proxy (HTTP) → <code>127.0.0.1:8080</code></div></div>
            </div>
        </div>

        <footer>Freedom Network v1.0 &nbsp;|&nbsp; GNU AGPLv3 &nbsp;|&nbsp; <a href="https://github.com/ayobro1/freedom-network" target="_blank">GitHub</a></footer>
    </div>

    <script>
        function formatBytes(bytes) {
            if (bytes === 0) return '0 B';
            const k = 1024, sizes = ['B', 'KB', 'MB', 'GB'];
            const i = Math.floor(Math.log(bytes) / Math.log(k));
            return (bytes / Math.pow(k, i)).toFixed(1) + ' ' + sizes[i];
        }
        function formatUptime(ms) {
            const s = Math.floor(ms / 1000), m = Math.floor(s / 60), h = Math.floor(m / 60), d = Math.floor(h / 24);
            if (d > 0) return d + 'd ' + (h % 24) + 'h';
            if (h > 0) return h + 'h ' + (m % 60) + 'm';
            if (m > 0) return m + 'm ' + (s % 60) + 's';
            return s + 's';
        }
        async function refresh() {
            try {
                const [sr, dr] = await Promise.all([fetch('/api/status'), fetch('/api/stats')]);
                const s = await sr.json(), d = await dr.json();
                document.getElementById('uptime').textContent = formatUptime(s.uptime_ms);
                document.getElementById('connections').textContent = s.connections_active;
                document.getElementById('totalconns').textContent = s.connections_total;
                document.getElementById('sent').textContent = formatBytes(d.bytes_sent);
                document.getElementById('recv').textContent = formatBytes(d.bytes_received);
                document.getElementById('total').textContent = formatBytes(d.bytes_sent + d.bytes_received);
            } catch(e) { console.error(e); }
        }
        setInterval(refresh, 1000);
        refresh();
    </script>
</body>
</html>"#;

        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
            html.len(),
            html
        )
    }

    fn cors_json_response(json: &str) -> String {
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\n\r\n{}",
            json.len(),
            json
        )
    }

    async fn api_status(proxy_metrics: &ProxyMetrics, start_time: SystemTime) -> String {
        let uptime_ms = start_time.elapsed().unwrap_or_default().as_millis() as u64;
        let active = *proxy_metrics.active_connections.read().await;
        let total = *proxy_metrics.total_connections.read().await;
        
        let json = format!(
            r#"{{"status":"running","uptime_ms":{},"connections_active":{},"connections_total":{}}}"#,
            uptime_ms, active, total,
        );
        
        Self::cors_json_response(&json)
    }

    async fn api_config() -> String {
        let json = r#"{"proxy_enabled":true,"proxy_address":"127.0.0.1:8080","quic_address":"127.0.0.1:5000","dashboard_address":"127.0.0.1:9090","dht_enabled":true,"onion_routing":true}"#;
        Self::cors_json_response(json)
    }

    async fn api_stats(proxy_metrics: &ProxyMetrics) -> String {
        let sent = *proxy_metrics.bytes_sent.read().await;
        let recv = *proxy_metrics.bytes_received.read().await;
        let total_conn = *proxy_metrics.total_connections.read().await;
        let active_conn = *proxy_metrics.active_connections.read().await;
        
        let json = format!(
            r#"{{"bytes_sent":{},"bytes_received":{},"connections_total":{},"connections_active":{}}}"#,
            sent, recv, total_conn, active_conn,
        );
        
        Self::cors_json_response(&json)
    }

    async fn not_found() -> String {
        "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n".to_string()
    }
}
