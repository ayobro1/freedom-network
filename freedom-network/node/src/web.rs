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
    <title>Freedom VPN</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        :root {
            --bg: #000000; --surface: #1c1c1e; --surface-2: #2c2c2e;
            --text: #f5f5f7; --muted: #a1a1aa; --accent: #0a84ff;
            --ok: #30d158; --bad: #ff453a; --border: #3a3a3c;
        }
        body { font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', 'SF Pro Display', 'Segoe UI', sans-serif; background: var(--bg); color: var(--text); min-height: 100vh; }
        .shell { max-width: 980px; margin: 0 auto; padding: 28px 20px 36px; }
        .topbar { display: flex; justify-content: space-between; align-items: center; margin-bottom: 28px; }
        .status-pill { display: inline-flex; align-items: center; gap: 8px; background: var(--surface); border: 1px solid var(--border); border-radius: 999px; padding: 8px 14px; font-size: 13px; }
        .status-dot { width: 8px; height: 8px; border-radius: 999px; background: var(--bad); }
        .status-dot.online { background: var(--ok); box-shadow: 0 0 0 6px rgba(48, 209, 88, 0.16); }
        h1 { font-size: 34px; font-weight: 700; margin-bottom: 6px; }
        .subtitle { color: var(--muted); margin-bottom: 24px; font-size: 14px; }
        .power-wrap { display: flex; justify-content: center; margin: 12px 0 24px; }
        .power-ring { width: 200px; height: 200px; border-radius: 50%; background: conic-gradient(from 0deg, var(--bad), #632525 70%); display: grid; place-items: center; }
        .power-ring.online { background: conic-gradient(from 0deg, var(--ok), #216132 70%); }
        .power-inner { width: 156px; height: 156px; border-radius: 50%; background: var(--surface); border: 1px solid var(--border); display: grid; place-items: center; font-size: 18px; font-weight: 700; }
        .stats-row { display: grid; grid-template-columns: repeat(5, minmax(0, 1fr)); gap: 10px; margin-bottom: 20px; }
        .stat-card { background: var(--surface); border: 1px solid var(--border); border-radius: 12px; padding: 12px; display: flex; flex-direction: column; gap: 6px; }
        .stat-label { color: var(--muted); font-size: 11px; text-transform: uppercase; letter-spacing: 0.4px; }
        .stat-value { color: var(--accent); font-size: 17px; font-weight: 700; }
        .proxy-card { background: var(--surface); border: 1px solid var(--border); border-radius: 14px; padding: 18px; }
        .proxy-card h2 { font-size: 14px; text-transform: uppercase; letter-spacing: 0.5px; margin-bottom: 12px; color: var(--accent); }
        .endpoint-row { display: flex; justify-content: space-between; align-items: center; padding: 10px 0; border-bottom: 1px solid var(--border); font-size: 13px; }
        .endpoint-row:last-of-type { margin-bottom: 10px; }
        code { background: var(--surface-2); border: 1px solid var(--border); border-radius: 6px; padding: 2px 7px; color: #8ec8ff; font-size: 12px; }
        ol { margin-left: 18px; color: var(--muted); font-size: 13px; line-height: 1.6; }
        a { color: #8ec8ff; }
        @media (max-width: 900px) { .stats-row { grid-template-columns: repeat(2, minmax(0, 1fr)); } }
    </style>
</head>
<body>
    <div class="shell">
        <div class="topbar">
            <div class="status-pill"><span class="status-dot" id="status-dot"></span><span id="connection-status">Offline</span></div>
        </div>
        <h1>Freedom VPN</h1>
        <p class="subtitle">Private routing and local proxy metrics.</p>

        <div class="power-wrap">
            <div class="power-ring" id="power-ring"><div class="power-inner" id="power-state">OFFLINE</div></div>
        </div>

        <section class="stats-row">
            <article class="stat-card"><span class="stat-label">Uptime</span><span class="stat-value" id="uptime">—</span></article>
            <article class="stat-card"><span class="stat-label">Active</span><span class="stat-value" id="connections">—</span></article>
            <article class="stat-card"><span class="stat-label">Total</span><span class="stat-value" id="totalconns">—</span></article>
            <article class="stat-card"><span class="stat-label">Sent</span><span class="stat-value" id="sent">—</span></article>
            <article class="stat-card"><span class="stat-label">Received</span><span class="stat-value" id="recv">—</span></article>
        </section>

        <section class="proxy-card">
            <h2>Proxy Setup</h2>
            <div class="endpoint-row"><span>HTTP Proxy</span><code>127.0.0.1:8080</code></div>
            <div class="endpoint-row"><span>QUIC Server</span><code>127.0.0.1:5000</code></div>
            <div class="endpoint-row"><span>Dashboard API</span><code>127.0.0.1:9090</code></div>
            <ol>
                <li>Open browser network/proxy settings.</li>
                <li>Set HTTP proxy to <code>127.0.0.1</code> and port <code>8080</code>.</li>
                <li>Keep Freedom VPN running while browsing.</li>
            </ol>
        </section>

        <footer style="margin-top:16px;color:var(--muted);font-size:12px;">Freedom VPN Dashboard &nbsp;|&nbsp; <a href="https://github.com/ayobro1/freedom-network" target="_blank">GitHub</a></footer>
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
        function setOnline(online) {
            const dot = document.getElementById('status-dot');
            const status = document.getElementById('connection-status');
            const ring = document.getElementById('power-ring');
            const state = document.getElementById('power-state');
            if (online) {
                dot.classList.add('online');
                status.textContent = 'Online';
                ring.classList.add('online');
                state.textContent = 'ONLINE';
            } else {
                dot.classList.remove('online');
                status.textContent = 'Offline';
                ring.classList.remove('online');
                state.textContent = 'OFFLINE';
            }
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
                setOnline(true);
            } catch(e) {
                setOnline(false);
                console.error(e);
            }
        }
        setInterval(refresh, 2000);
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
