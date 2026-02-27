const refreshBtn = document.getElementById('refresh-btn');
const connectBtn = document.getElementById('connect-btn');
const saveSettingsBtn = document.getElementById('save-settings-btn');
const statusDot = document.getElementById('status-dot');
const connectionStatus = document.getElementById('connection-status');
const powerRing = document.getElementById('power-ring');
const powerState = document.getElementById('power-state');
const modeSelect = document.getElementById('mode-select');
const locationSelect = document.getElementById('location-select');
const protocolSelect = document.getElementById('protocol-select');
const killSwitch = document.getElementById('kill-switch');
const threatProtection = document.getElementById('threat-protection');
const dnsProtection = document.getElementById('dns-protection');
const strictEnforcement = document.getElementById('strict-enforcement');
const autoConnect = document.getElementById('auto-connect');
const autoConnectRule = document.getElementById('auto-connect-rule');
const splitTunnelApps = document.getElementById('split-tunnel-apps');
const syncSplitBtn = document.getElementById('sync-split-btn');
const stopSplitBtn = document.getElementById('stop-split-btn');
const splitRunning = document.getElementById('split-running');
const runDiagnosticsBtn = document.getElementById('run-diagnostics-btn');
const diagnosticsOutput = document.getElementById('diagnostics-output');
const exportProfileBtn = document.getElementById('export-profile-btn');
const importProfileFile = document.getElementById('import-profile-file');

const NODE_API = 'http://127.0.0.1:9090';
const STAT_IDS = ['stat-uptime', 'stat-active', 'stat-total', 'stat-sent', 'stat-recv'];
const tauriInvoke = window.__TAURI__?.core?.invoke;

let vpnConnected = false;
let strictEnforcementActive = false;

refreshBtn.addEventListener('click', () => {
    refreshBtn.disabled = true;
    fetchNodeStats();
    setTimeout(() => {
        refreshBtn.disabled = false;
    }, 350);
});

function formatBytes(bytes) {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return (bytes / Math.pow(k, i)).toFixed(1) + ' ' + sizes[i];
}

function formatUptime(ms) {
    const seconds = Math.floor(ms / 1000);
    const minutes = Math.floor(seconds / 60);
    const hours = Math.floor(minutes / 60);
    const days = Math.floor(hours / 24);
    if (days > 0) return days + 'd ' + (hours % 24) + 'h';
    if (hours > 0) return hours + 'h ' + (minutes % 60) + 'm';
    if (minutes > 0) return minutes + 'm ' + (seconds % 60) + 's';
    return seconds + 's';
}

function setNodeOnline(online) {
    if (online) {
        statusDot.classList.add('online');
        if (strictEnforcementActive) {
            connectionStatus.textContent = 'Protected+Intercept';
        } else {
            connectionStatus.textContent = vpnConnected ? 'Protected' : 'Online';
        }
        powerRing.classList.add('online');
        powerState.textContent = vpnConnected ? 'PROTECTED' : 'ONLINE';
    } else {
        statusDot.classList.remove('online');
        connectionStatus.textContent = 'Offline';
        powerRing.classList.remove('online');
        powerState.textContent = 'OFFLINE';
    }
}

function getSettingsPayload() {
    return {
        mode: modeSelect.value,
        protocol: protocolSelect.value,
        location: locationSelect.value,
        strictEnforcement: strictEnforcement.checked,
        killSwitch: killSwitch.checked,
        threatProtection: threatProtection.checked,
        dnsProtection: dnsProtection.checked,
        autoConnect: autoConnect.checked,
        autoConnectRule: autoConnectRule.value,
        splitTunnelApps: splitTunnelApps.value
            .split(/\r?\n/)
            .map(v => v.trim())
            .filter(Boolean)
    };
}

function applySettingsToForm(settings) {
    modeSelect.value = settings.mode || 'full';
    protocolSelect.value = settings.protocol || 'auto';
    locationSelect.value = settings.location || 'fastest';
    strictEnforcement.checked = !!settings.strictEnforcement;
    killSwitch.checked = !!settings.killSwitch;
    threatProtection.checked = !!settings.threatProtection;
    dnsProtection.checked = !!settings.dnsProtection;
    autoConnect.checked = !!settings.autoConnect;
    autoConnectRule.value = settings.autoConnectRule || 'startup';
    splitTunnelApps.value = Array.isArray(settings.splitTunnelApps)
        ? settings.splitTunnelApps.join('\n')
        : '';
}

function updateConnectButton() {
    connectBtn.textContent = vpnConnected ? 'Disconnect' : 'Connect';
}

function updateSplitRunning(status) {
    const running = Array.isArray(status?.splitTunnelRunning)
        ? status.splitTunnelRunning
        : [];
    splitRunning.textContent = running.length
        ? `Split apps running: ${running.join(', ')}`
        : 'Split apps running: none';
}

async function syncVpnStatus() {
    if (!tauriInvoke) return;
    try {
        const status = await tauriInvoke('get_vpn_status');
        vpnConnected = !!status.connected;
        strictEnforcementActive = !!status.strictEnforcementActive;
        applySettingsToForm(status.settings || {});
        updateSplitRunning(status);
        updateConnectButton();
    } catch (_) {
        vpnConnected = false;
        updateConnectButton();
    }
}

async function loadLocationProfiles() {
    if (!tauriInvoke) return;
    try {
        const profiles = await tauriInvoke('get_location_profiles');
        const current = locationSelect.value;
        locationSelect.innerHTML = '';
        profiles.forEach(profile => {
            const option = document.createElement('option');
            option.value = profile.id;
            option.textContent = `${profile.label} (${profile.region})`;
            locationSelect.appendChild(option);
        });
        locationSelect.value = current || 'fastest';
    } catch (_) {
        // ignore profile loading failures in non-tauri contexts
    }
}

async function toggleVpnConnection() {
    if (!tauriInvoke) {
        alert('Desktop controls require the Tauri desktop app runtime.');
        return;
    }

    connectBtn.disabled = true;
    try {
        const command = vpnConnected ? 'stop_vpn' : 'start_vpn';
        const status = await tauriInvoke(command);
        vpnConnected = !!status.connected;
        strictEnforcementActive = !!status.strictEnforcementActive;
        updateSplitRunning(status);
        updateConnectButton();
        await fetchNodeStats();
    } catch (error) {
        alert(`VPN action failed: ${error}`);
    } finally {
        connectBtn.disabled = false;
    }
}

async function saveVpnSettings() {
    if (!tauriInvoke) {
        alert('Desktop controls require the Tauri desktop app runtime.');
        return;
    }

    saveSettingsBtn.disabled = true;
    try {
        const status = await tauriInvoke('update_vpn_settings', {
            settings: getSettingsPayload()
        });
        vpnConnected = !!status.connected;
        strictEnforcementActive = !!status.strictEnforcementActive;
        updateSplitRunning(status);
        updateConnectButton();
    } catch (error) {
        alert(`Failed to save settings: ${error}`);
    } finally {
        saveSettingsBtn.disabled = false;
    }
}

async function syncSplitTunnelApps() {
    if (!tauriInvoke) {
        alert('Split tunneling controls require the Tauri desktop app runtime.');
        return;
    }

    syncSplitBtn.disabled = true;
    try {
        const status = await tauriInvoke('sync_split_tunnel_apps');
        strictEnforcementActive = !!status.strictEnforcementActive;
        updateSplitRunning(status);
    } catch (error) {
        alert(`Failed to apply split tunnel apps: ${error}`);
    } finally {
        syncSplitBtn.disabled = false;
    }
}

async function stopSplitTunnelApps() {
    if (!tauriInvoke) {
        alert('Split tunneling controls require the Tauri desktop app runtime.');
        return;
    }

    stopSplitBtn.disabled = true;
    try {
        const status = await tauriInvoke('stop_split_tunnel_apps');
        strictEnforcementActive = !!status.strictEnforcementActive;
        updateSplitRunning(status);
    } catch (error) {
        alert(`Failed to stop split tunnel apps: ${error}`);
    } finally {
        stopSplitBtn.disabled = false;
    }
}

function formatDiagBool(value) {
    return value ? 'OK' : 'FAIL';
}

async function runDiagnostics() {
    if (!tauriInvoke) {
        diagnosticsOutput.textContent = 'Diagnostics require the Tauri desktop app runtime.';
        return;
    }

    runDiagnosticsBtn.disabled = true;
    diagnosticsOutput.textContent = 'Running diagnostics...';
    try {
        const diag = await tauriInvoke('run_diagnostics');
        diagnosticsOutput.textContent = [
            `Dashboard: ${formatDiagBool(diag.dashboardReachable)} (${diag.dashboardLatencyMs ?? '-'} ms)`,
            `Proxy: ${formatDiagBool(diag.proxyReachable)} (${diag.proxyLatencyMs ?? '-'} ms)`,
            `QUIC: ${formatDiagBool(diag.quicReachable)} (${diag.quicLatencyMs ?? '-'} ms)`,
            `Sample duration: ${diag.timestampMs} ms`
        ].join('\n');
    } catch (error) {
        diagnosticsOutput.textContent = `Diagnostics failed: ${error}`;
    } finally {
        runDiagnosticsBtn.disabled = false;
    }
}

async function exportProfile() {
    if (!tauriInvoke) {
        alert('Profile export requires the Tauri desktop app runtime.');
        return;
    }

    try {
        const profileJson = await tauriInvoke('export_vpn_profile');
        const blob = new Blob([profileJson], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        const link = document.createElement('a');
        link.href = url;
        link.download = 'freedom-vpn-profile.json';
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
        URL.revokeObjectURL(url);
    } catch (error) {
        alert(`Failed to export profile: ${error}`);
    }
}

async function importProfile(event) {
    if (!tauriInvoke) {
        alert('Profile import requires the Tauri desktop app runtime.');
        return;
    }

    const file = event.target.files?.[0];
    if (!file) return;

    try {
        const profileJson = await file.text();
        const status = await tauriInvoke('import_vpn_profile', { profileJson });
        vpnConnected = !!status.connected;
        applySettingsToForm(status.settings || {});
        updateConnectButton();
        await saveVpnSettings();
    } catch (error) {
        alert(`Failed to import profile: ${error}`);
    } finally {
        importProfileFile.value = '';
    }
}

async function fetchNodeStats() {
    try {
        const [statusResp, statsResp] = await Promise.all([
            fetch(`${NODE_API}/api/status`),
            fetch(`${NODE_API}/api/stats`)
        ]);

        if (!statusResp.ok || !statsResp.ok) throw new Error('Bad response');

        const status = await statusResp.json();
        const stats = await statsResp.json();

        document.getElementById('stat-uptime').textContent = formatUptime(status.uptime_ms);
        document.getElementById('stat-active').textContent = status.connections_active;
        document.getElementById('stat-total').textContent = status.connections_total;
        document.getElementById('stat-sent').textContent = formatBytes(stats.bytes_sent);
        document.getElementById('stat-recv').textContent = formatBytes(stats.bytes_received);

        setNodeOnline(true);
    } catch (e) {
        setNodeOnline(vpnConnected);
        STAT_IDS.forEach(id => {
            const el = document.getElementById(id);
            if (el) el.textContent = '—';
        });
    }
}

connectBtn.addEventListener('click', toggleVpnConnection);
saveSettingsBtn.addEventListener('click', saveVpnSettings);
syncSplitBtn.addEventListener('click', syncSplitTunnelApps);
stopSplitBtn.addEventListener('click', stopSplitTunnelApps);
runDiagnosticsBtn.addEventListener('click', runDiagnostics);
exportProfileBtn.addEventListener('click', exportProfile);
importProfileFile.addEventListener('change', importProfile);

loadLocationProfiles();
syncVpnStatus();
fetchNodeStats();
setInterval(fetchNodeStats, 2000);
