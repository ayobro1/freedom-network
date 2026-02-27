#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use fdom::FdomProcessor;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::time::Instant;

#[tauri::command]
async fn fetch_freedom_site(domain: String, path: String) -> Result<String, String> {
    // This will fetch from the freedom network node running on 127.0.0.1:5000
    let url = format!("http://127.0.0.1:5000/site/{}/{}", domain, path);
    
    match reqwest::get(&url).await {
        Ok(response) => {
            match response.text().await {
                Ok(content) => {
                    // If it's a .fdom file, render it to HTML
                    if path.ends_with(".fdom") {
                        match FdomProcessor::process(&content) {
                            Ok(html) => Ok(html),
                            Err(e) => Err(format!("Failed to render .fdom: {}", e)),
                        }
                    } else {
                        Ok(content)
                    }
                }
                Err(e) => Err(format!("Failed to read response: {}", e)),
            }
        }
        Err(e) => Err(format!("Failed to fetch from network: {}", e)),
    }
}

#[tauri::command]
fn get_node_status() -> String {
    "Freedom Network Node running on 127.0.0.1:5000".to_string()
}

#[tauri::command]
fn render_fdom(fdom_source: String) -> Result<String, String> {
    FdomProcessor::process(&fdom_source)
        .map_err(|e| format!("Failed to render .fdom: {}", e))
}

#[tauri::command]
fn load_fdom_file(file_path: String) -> Result<String, String> {
    // Load .fdom file from filesystem
    let path = Path::new(&file_path);
    
    if !path.exists() {
        return Err(format!("File not found: {}", file_path));
    }
    
    if !path.extension().and_then(|ext| ext.to_str()).map_or(false, |ext| ext == "fdom") {
        return Err("File must be a .fdom file".to_string());
    }
    
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    FdomProcessor::process(&content)
        .map_err(|e| format!("Failed to render .fdom: {}", e))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VpnSettings {
    mode: String,
    protocol: String,
    location: String,
    strict_enforcement: bool,
    kill_switch: bool,
    threat_protection: bool,
    dns_protection: bool,
    auto_connect: bool,
    auto_connect_rule: String,
    split_tunnel_apps: Vec<String>,
}

impl Default for VpnSettings {
    fn default() -> Self {
        Self {
            mode: "full".to_string(),
            protocol: "auto".to_string(),
            location: "fastest".to_string(),
            strict_enforcement: false,
            kill_switch: true,
            threat_protection: true,
            dns_protection: true,
            auto_connect: false,
            auto_connect_rule: "startup".to_string(),
            split_tunnel_apps: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct LocationProfile {
    id: String,
    label: String,
    region: String,
    recommended: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct DiagnosticsResult {
    timestamp_ms: u128,
    dashboard_reachable: bool,
    proxy_reachable: bool,
    quic_reachable: bool,
    dashboard_latency_ms: Option<u128>,
    proxy_latency_ms: Option<u128>,
    quic_latency_ms: Option<u128>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct VpnStatus {
    connected: bool,
    system_proxy_enabled: bool,
    kill_switch_active: bool,
    strict_enforcement_active: bool,
    node_binary: Option<String>,
    split_tunnel_running: Vec<String>,
    proxy_address: String,
    dashboard_api: String,
    settings: VpnSettings,
}

struct ManagedSplitApp {
    entry: String,
    child: Child,
}

#[derive(Default)]
struct RuntimeState {
    node_child: Option<Child>,
    node_binary: Option<PathBuf>,
    system_proxy_enabled: bool,
    kill_switch_active: bool,
    strict_enforcement_active: bool,
    split_apps: Vec<ManagedSplitApp>,
    settings: VpnSettings,
}

struct AppState {
    runtime: Mutex<RuntimeState>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            runtime: Mutex::new(RuntimeState::default()),
        }
    }
}

fn find_node_binary() -> Option<PathBuf> {
    let mut candidates = Vec::new();
    let exe_name = if cfg!(target_os = "windows") {
        "freedom-node.exe"
    } else {
        "freedom-node"
    };

    if let Ok(current_exe) = std::env::current_exe() {
        if let Some(exe_dir) = current_exe.parent() {
            candidates.push(exe_dir.join(exe_name));
            candidates.push(exe_dir.join("..").join(exe_name));
            candidates.push(exe_dir.join("..").join("..").join(exe_name));
        }
    }

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    candidates.push(manifest_dir.join("../../node/target/release").join(exe_name));
    candidates.push(manifest_dir.join("../../../dist-windows").join(exe_name));

    candidates.into_iter().find(|path| path.exists())
}

fn apply_system_proxy(enable: bool, proxy: &str) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let ps_script = if enable {
            format!(
                "$path='HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings'; \
                 Set-ItemProperty -Path $path -Name ProxyEnable -Type DWord -Value 1; \
                 Set-ItemProperty -Path $path -Name ProxyServer -Value '{proxy}'; \
                 netsh winhttp set proxy {proxy} | Out-Null"
            )
        } else {
            "$path='HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings'; \
             Set-ItemProperty -Path $path -Name ProxyEnable -Type DWord -Value 0; \
             netsh winhttp reset proxy | Out-Null"
                .to_string()
        };

        let status = Command::new("powershell")
            .arg("-NoProfile")
            .arg("-ExecutionPolicy")
            .arg("Bypass")
            .arg("-Command")
            .arg(ps_script)
            .status()
            .map_err(|e| format!("Failed to apply system proxy settings: {e}"))?;

        if !status.success() {
            return Err("PowerShell failed while applying system proxy settings".to_string());
        }

        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = (enable, proxy);
        Err("System-wide proxy mode is currently supported on Windows desktop builds".to_string())
    }
}

fn apply_kill_switch_block(enable: bool) -> Result<(), String> {
    if enable {
        apply_system_proxy(true, "127.0.0.1:9")
    } else {
        apply_system_proxy(false, "127.0.0.1:8080")
    }
}

fn resolve_executable(entry: &str) -> Result<String, String> {
    let trimmed = entry.trim();
    if trimmed.is_empty() {
        return Err("Empty split tunnel app entry".to_string());
    }

    let path = PathBuf::from(trimmed);
    if path.exists() {
        return Ok(trimmed.to_string());
    }

    #[cfg(target_os = "windows")]
    {
        let output = Command::new("where")
            .arg(trimmed)
            .output()
            .map_err(|e| format!("Failed to resolve executable {trimmed}: {e}"))?;
        if output.status.success() {
            let resolved = String::from_utf8_lossy(&output.stdout)
                .lines()
                .next()
                .unwrap_or(trimmed)
                .trim()
                .to_string();
            return Ok(resolved);
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let output = Command::new("which")
            .arg(trimmed)
            .output()
            .map_err(|e| format!("Failed to resolve executable {trimmed}: {e}"))?;
        if output.status.success() {
            let resolved = String::from_utf8_lossy(&output.stdout)
                .lines()
                .next()
                .unwrap_or(trimmed)
                .trim()
                .to_string();
            return Ok(resolved);
        }
    }

    Err(format!("Executable not found: {trimmed}"))
}

fn stop_managed_split_apps(runtime: &mut RuntimeState) {
    for managed in &mut runtime.split_apps {
        let _ = managed.child.kill();
        let _ = managed.child.wait();
    }
    runtime.split_apps.clear();
}

fn escape_ps_single_quoted(value: &str) -> String {
    value.replace('"', "").replace('\'', "''")
}

fn apply_windows_firewall_policy(enable: bool, node_binary: Option<&PathBuf>, split_apps: &[String], full_mode: bool) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let mut script = String::new();
        script.push_str("$ErrorActionPreference='Stop'; ");
        script.push_str("Get-NetFirewallRule -DisplayName 'FreedomVPN-*' -ErrorAction SilentlyContinue | Remove-NetFirewallRule -ErrorAction SilentlyContinue; ");

        if enable {
            if full_mode {
                script.push_str("New-NetFirewallRule -DisplayName 'FreedomVPN-Global-Block-Internet' -Direction Outbound -Action Block -RemoteAddress Internet -Profile Any | Out-Null; ");
                if let Some(node) = node_binary {
                    let node_str = escape_ps_single_quoted(&node.display().to_string());
                    script.push_str(&format!(
                        "New-NetFirewallRule -DisplayName 'FreedomVPN-Node-Allow' -Direction Outbound -Action Allow -Program '{}' -Profile Any | Out-Null; ",
                        node_str
                    ));
                }
            } else {
                for (index, entry) in split_apps.iter().enumerate() {
                    if let Ok(exe) = resolve_executable(entry) {
                        let exe_str = escape_ps_single_quoted(&exe);
                        script.push_str(&format!(
                            "New-NetFirewallRule -DisplayName 'FreedomVPN-Split-{}' -Direction Outbound -Action Block -Program '{}' -RemoteAddress Internet -Profile Any | Out-Null; ",
                            index,
                            exe_str
                        ));
                    }
                }
            }
        }

        let status = Command::new("powershell")
            .arg("-NoProfile")
            .arg("-ExecutionPolicy")
            .arg("Bypass")
            .arg("-Command")
            .arg(script)
            .status()
            .map_err(|e| format!("Failed to apply Windows firewall policy: {e}"))?;

        if !status.success() {
            return Err("PowerShell failed while applying Windows firewall policy (admin rights may be required).".to_string());
        }

        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = (enable, node_binary, split_apps, full_mode);
        Err("Strict system interception is currently supported on Windows desktop builds".to_string())
    }
}

fn sync_enforcement_policy(runtime: &mut RuntimeState, keep_block_on_disconnect: bool) -> Result<(), String> {
    if !runtime.settings.strict_enforcement {
        apply_windows_firewall_policy(false, runtime.node_binary.as_ref(), &runtime.settings.split_tunnel_apps, runtime.settings.mode == "full")?;
        runtime.strict_enforcement_active = false;
        return Ok(());
    }

    let full_mode = runtime.settings.mode == "full";
    let connected = runtime.node_child.is_some();
    let should_enable = if full_mode {
        connected || keep_block_on_disconnect
    } else {
        connected
    };

    apply_windows_firewall_policy(
        should_enable,
        runtime.node_binary.as_ref(),
        &runtime.settings.split_tunnel_apps,
        full_mode,
    )?;
    runtime.strict_enforcement_active = should_enable;
    Ok(())
}

fn start_managed_split_apps(runtime: &mut RuntimeState) -> Result<(), String> {
    stop_managed_split_apps(runtime);

    if runtime.settings.mode != "app" {
        return Ok(());
    }

    for entry in runtime.settings.split_tunnel_apps.clone() {
        let executable = resolve_executable(&entry)?;
        let child = Command::new(&executable)
            .env("HTTP_PROXY", "http://127.0.0.1:8080")
            .env("HTTPS_PROXY", "http://127.0.0.1:8080")
            .env("ALL_PROXY", "http://127.0.0.1:8080")
            .env("NO_PROXY", "localhost,127.0.0.1")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("Failed to start split tunnel app {entry}: {e}"))?;

        runtime.split_apps.push(ManagedSplitApp { entry, child });
    }

    Ok(())
}

fn location_profiles() -> Vec<LocationProfile> {
    vec![
        LocationProfile {
            id: "fastest".to_string(),
            label: "Fastest".to_string(),
            region: "Auto".to_string(),
            recommended: true,
        },
        LocationProfile {
            id: "nearest".to_string(),
            label: "Nearest".to_string(),
            region: "Auto".to_string(),
            recommended: false,
        },
        LocationProfile {
            id: "us-east".to_string(),
            label: "US East".to_string(),
            region: "United States".to_string(),
            recommended: false,
        },
        LocationProfile {
            id: "us-west".to_string(),
            label: "US West".to_string(),
            region: "United States".to_string(),
            recommended: false,
        },
        LocationProfile {
            id: "eu-central".to_string(),
            label: "EU Central".to_string(),
            region: "Europe".to_string(),
            recommended: false,
        },
        LocationProfile {
            id: "asia-sg".to_string(),
            label: "Asia Singapore".to_string(),
            region: "Asia".to_string(),
            recommended: false,
        },
    ]
}

async fn tcp_probe(addr: &str) -> (bool, Option<u128>) {
    let started = Instant::now();
    match tokio::net::TcpStream::connect(addr).await {
        Ok(_) => (true, Some(started.elapsed().as_millis())),
        Err(_) => (false, None),
    }
}

fn snapshot_status(runtime: &RuntimeState) -> VpnStatus {
    VpnStatus {
        connected: runtime.node_child.is_some(),
        system_proxy_enabled: runtime.system_proxy_enabled,
        kill_switch_active: runtime.kill_switch_active,
        strict_enforcement_active: runtime.strict_enforcement_active,
        node_binary: runtime
            .node_binary
            .as_ref()
            .map(|path| path.display().to_string()),
        split_tunnel_running: runtime
            .split_apps
            .iter()
            .map(|a| a.entry.clone())
            .collect(),
        proxy_address: "127.0.0.1:8080".to_string(),
        dashboard_api: "http://127.0.0.1:9090".to_string(),
        settings: runtime.settings.clone(),
    }
}

#[tauri::command]
fn get_vpn_status(state: tauri::State<'_, AppState>) -> Result<VpnStatus, String> {
    let runtime = state
        .runtime
        .lock()
        .map_err(|_| "Failed to lock VPN runtime state".to_string())?;
    Ok(snapshot_status(&runtime))
}

#[tauri::command]
fn update_vpn_settings(settings: VpnSettings, state: tauri::State<'_, AppState>) -> Result<VpnStatus, String> {
    let mut runtime = state
        .runtime
        .lock()
        .map_err(|_| "Failed to lock VPN runtime state".to_string())?;

    runtime.settings = settings;

    if runtime.node_child.is_some() {
        let should_enable_proxy = runtime.settings.mode == "full";
        if should_enable_proxy != runtime.system_proxy_enabled {
            apply_system_proxy(should_enable_proxy, "127.0.0.1:8080")?;
            runtime.system_proxy_enabled = should_enable_proxy;
        }
        start_managed_split_apps(&mut runtime)?;
        sync_enforcement_policy(&mut runtime, false)?;
        runtime.kill_switch_active = false;
    } else {
        sync_enforcement_policy(&mut runtime, runtime.kill_switch_active)?;
    }

    Ok(snapshot_status(&runtime))
}

#[tauri::command]
fn sync_split_tunnel_apps(state: tauri::State<'_, AppState>) -> Result<VpnStatus, String> {
    let mut runtime = state
        .runtime
        .lock()
        .map_err(|_| "Failed to lock VPN runtime state".to_string())?;

    if runtime.node_child.is_some() {
        start_managed_split_apps(&mut runtime)?;
        sync_enforcement_policy(&mut runtime, false)?;
    }

    Ok(snapshot_status(&runtime))
}

#[tauri::command]
fn stop_split_tunnel_apps(state: tauri::State<'_, AppState>) -> Result<VpnStatus, String> {
    let mut runtime = state
        .runtime
        .lock()
        .map_err(|_| "Failed to lock VPN runtime state".to_string())?;

    stop_managed_split_apps(&mut runtime);
    sync_enforcement_policy(&mut runtime, false)?;
    Ok(snapshot_status(&runtime))
}

#[tauri::command]
fn get_location_profiles() -> Vec<LocationProfile> {
    location_profiles()
}

#[tauri::command]
async fn run_diagnostics() -> Result<DiagnosticsResult, String> {
    let started = Instant::now();
    let (dashboard_reachable, dashboard_latency_ms) = tcp_probe("127.0.0.1:9090").await;
    let (proxy_reachable, proxy_latency_ms) = tcp_probe("127.0.0.1:8080").await;
    let (quic_reachable, quic_latency_ms) = tcp_probe("127.0.0.1:5000").await;

    Ok(DiagnosticsResult {
        timestamp_ms: started.elapsed().as_millis(),
        dashboard_reachable,
        proxy_reachable,
        quic_reachable,
        dashboard_latency_ms,
        proxy_latency_ms,
        quic_latency_ms,
    })
}

#[tauri::command]
fn export_vpn_profile(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let runtime = state
        .runtime
        .lock()
        .map_err(|_| "Failed to lock VPN runtime state".to_string())?;

    serde_json::to_string_pretty(&runtime.settings)
        .map_err(|e| format!("Failed to export profile: {e}"))
}

#[tauri::command]
fn import_vpn_profile(profile_json: String, state: tauri::State<'_, AppState>) -> Result<VpnStatus, String> {
    let imported: VpnSettings = serde_json::from_str(&profile_json)
        .map_err(|e| format!("Invalid profile JSON: {e}"))?;

    let mut runtime = state
        .runtime
        .lock()
        .map_err(|_| "Failed to lock VPN runtime state".to_string())?;

    runtime.settings = imported;
    Ok(snapshot_status(&runtime))
}

#[tauri::command]
fn start_vpn(state: tauri::State<'_, AppState>) -> Result<VpnStatus, String> {
    let mut runtime = state
        .runtime
        .lock()
        .map_err(|_| "Failed to lock VPN runtime state".to_string())?;

    if runtime.node_child.is_some() {
        return Ok(snapshot_status(&runtime));
    }

    let node_binary = find_node_binary().ok_or_else(|| {
        "Could not find freedom-node binary. Build node first or place freedom-node.exe next to the app."
            .to_string()
    })?;

    let child = Command::new(&node_binary)
        .env(
            "FREEDOM_THREAT_PROTECTION",
            if runtime.settings.threat_protection { "1" } else { "0" },
        )
        .env(
            "FREEDOM_DNS_PROTECTION",
            if runtime.settings.dns_protection { "1" } else { "0" },
        )
        .env(
            "FREEDOM_SPLIT_TUNNEL_APPS",
            runtime.settings.split_tunnel_apps.join(";"),
        )
        .env("FREEDOM_LOCATION", &runtime.settings.location)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Failed to start freedom-node: {e}"))?;

    runtime.node_binary = Some(node_binary);
    runtime.node_child = Some(child);

    if runtime.settings.mode == "full" {
        apply_system_proxy(true, "127.0.0.1:8080")?;
        runtime.system_proxy_enabled = true;
        runtime.kill_switch_active = false;
    } else {
        runtime.system_proxy_enabled = false;
        runtime.kill_switch_active = false;
    }

    start_managed_split_apps(&mut runtime)?;
    sync_enforcement_policy(&mut runtime, false)?;

    Ok(snapshot_status(&runtime))
}

#[tauri::command]
fn stop_vpn(state: tauri::State<'_, AppState>) -> Result<VpnStatus, String> {
    let mut runtime = state
        .runtime
        .lock()
        .map_err(|_| "Failed to lock VPN runtime state".to_string())?;

    if let Some(mut child) = runtime.node_child.take() {
        let _ = child.kill();
        let _ = child.wait();
    }

    stop_managed_split_apps(&mut runtime);

    let full_mode = runtime.settings.mode == "full";
    if full_mode && runtime.settings.kill_switch {
        if runtime.settings.strict_enforcement {
            runtime.system_proxy_enabled = false;
        } else {
            apply_kill_switch_block(true)?;
            runtime.system_proxy_enabled = true;
        }
        runtime.kill_switch_active = true;
    } else if runtime.system_proxy_enabled {
        apply_system_proxy(false, "127.0.0.1:8080")?;
        runtime.system_proxy_enabled = false;
        runtime.kill_switch_active = false;
    }

    sync_enforcement_policy(&mut runtime, runtime.kill_switch_active)?;

    Ok(snapshot_status(&runtime))
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            fetch_freedom_site,
            get_node_status,
            render_fdom,
            load_fdom_file,
            get_vpn_status,
            update_vpn_settings,
            get_location_profiles,
            run_diagnostics,
            export_vpn_profile,
            import_vpn_profile,
            sync_split_tunnel_apps,
            stop_split_tunnel_apps,
            start_vpn,
            stop_vpn
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
