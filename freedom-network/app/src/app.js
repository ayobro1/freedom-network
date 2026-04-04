/* ── DOM refs ── */
const refreshBtn = document.getElementById('refresh-btn');
const connectBtn = document.getElementById('connect-btn');
const saveSettingsBtn = document.getElementById('save-settings-btn');
const statusDot = document.getElementById('status-dot');
const connectionStatus = document.getElementById('connection-status');
const powerState = document.getElementById('power-state');
const connectTitle = document.getElementById('connect-title');
const topbarState = document.getElementById('topbar-state');
const viewTitle = document.getElementById('view-title');
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
const tauriInvoke = window.__TAURI__?.core?.invoke || window.__TAURI_INTERNALS__?.invoke;
const hasDesktopRuntime = typeof tauriInvoke === 'function';

let vpnConnected = false;
let strictEnforcementActive = false;

/* ── View switching ── */
const VIEW_TITLES = { connection: 'Connection', settings: 'Settings', diagnostics: 'Diagnostics' };

document.querySelectorAll('.nav-item[data-view]').forEach(function (btn) {
    btn.addEventListener('click', function () {
        var id = btn.getAttribute('data-view');
        document.querySelectorAll('.nav-item').forEach(function (n) { n.classList.remove('active'); });
        btn.classList.add('active');
        document.querySelectorAll('.view').forEach(function (v) { v.classList.remove('active'); });
        var target = document.getElementById('view-' + id);
        if (target) target.classList.add('active');
        viewTitle.textContent = VIEW_TITLES[id] || id;
    });
});

/* ── Helpers ── */
refreshBtn.addEventListener('click', function () {
    refreshBtn.disabled = true;
    fetchNodeStats();
    setTimeout(function () { refreshBtn.disabled = false; }, 350);
});

function formatBytes(bytes) {
    if (bytes === 0) return '0 B';
    var k = 1024;
    var sizes = ['B', 'KB', 'MB', 'GB'];
    var i = Math.floor(Math.log(bytes) / Math.log(k));
    return (bytes / Math.pow(k, i)).toFixed(1) + ' ' + sizes[i];
}

function formatUptime(ms) {
    var seconds = Math.floor(ms / 1000);
    var minutes = Math.floor(seconds / 60);
    var hours = Math.floor(minutes / 60);
    var days = Math.floor(hours / 24);
    if (days > 0) return days + 'd ' + (hours % 24) + 'h';
    if (hours > 0) return hours + 'h ' + (minutes % 60) + 'm';
    if (minutes > 0) return minutes + 'm ' + (seconds % 60) + 's';
    return seconds + 's';
}

function setNodeOnline(online) {
    if (online) {
        statusDot.classList.add('online');
        if (strictEnforcementActive) {
            connectionStatus.textContent = 'Protected + Intercept';
        } else {
            connectionStatus.textContent = vpnConnected ? 'Protected' : 'Online';
        }
        connectBtn.classList.add('connected');
        powerState.textContent = vpnConnected ? 'Protected' : 'Online';
        connectTitle.textContent = vpnConnected ? 'Connected — traffic routed' : 'Node online';
        topbarState.textContent = vpnConnected ? 'Protected' : 'Online';
    } else {
        statusDot.classList.remove('online');
        connectionStatus.textContent = 'Offline';
        connectBtn.classList.remove('connected');
        powerState.textContent = 'Connect';
        connectTitle.textContent = 'Not connected';
        topbarState.textContent = 'Disconnected';
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
            .map(function (v) { return v.trim(); })
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
    powerState.textContent = vpnConnected ? 'Disconnect' : 'Connect';
}

function updateSplitRunning(status) {
    var running = Array.isArray(status?.splitTunnelRunning)
        ? status.splitTunnelRunning
        : [];
    splitRunning.textContent = running.length
        ? 'Split apps running: ' + running.join(', ')
        : 'Split apps running: none';
}

function setRuntimeUiState() {
    if (hasDesktopRuntime) return;
    [
        connectBtn,
        saveSettingsBtn,
        syncSplitBtn,
        stopSplitBtn,
        runDiagnosticsBtn,
        exportProfileBtn,
        importProfileFile
    ].forEach((el) => {
        if (el) el.disabled = true;
    });
    topbarState.textContent = 'Browser Preview';
    diagnosticsOutput.textContent = 'Desktop-only actions are disabled in browser preview mode.';
}

/* ── Tauri commands ── */
async function syncVpnStatus() {
    if (!tauriInvoke) return;
    try {
        var status = await tauriInvoke('get_vpn_status');
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
        var profiles = await tauriInvoke('get_location_profiles');
        var current = locationSelect.value;
        locationSelect.innerHTML = '';
        profiles.forEach(function (profile) {
            var option = document.createElement('option');
            option.value = profile.id;
            option.textContent = profile.label + ' (' + profile.region + ')';
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
        var command = vpnConnected ? 'stop_vpn' : 'start_vpn';
        var status = await tauriInvoke(command);
        vpnConnected = !!status.connected;
        strictEnforcementActive = !!status.strictEnforcementActive;
        updateSplitRunning(status);
        updateConnectButton();
        await fetchNodeStats();
    } catch (error) {
        alert('VPN action failed: ' + error);
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
        var status = await tauriInvoke('update_vpn_settings', {
            settings: getSettingsPayload()
        });
        vpnConnected = !!status.connected;
        strictEnforcementActive = !!status.strictEnforcementActive;
        updateSplitRunning(status);
        updateConnectButton();
    } catch (error) {
        alert('Failed to save settings: ' + error);
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
        var status = await tauriInvoke('sync_split_tunnel_apps');
        strictEnforcementActive = !!status.strictEnforcementActive;
        updateSplitRunning(status);
    } catch (error) {
        alert('Failed to apply split tunnel apps: ' + error);
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
        var status = await tauriInvoke('stop_split_tunnel_apps');
        strictEnforcementActive = !!status.strictEnforcementActive;
        updateSplitRunning(status);
    } catch (error) {
        alert('Failed to stop split tunnel apps: ' + error);
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
        var diag = await tauriInvoke('run_diagnostics');
        diagnosticsOutput.textContent = [
            'Dashboard: ' + formatDiagBool(diag.dashboardReachable) + ' (' + (diag.dashboardLatencyMs ?? '-') + ' ms)',
            'Proxy:     ' + formatDiagBool(diag.proxyReachable) + ' (' + (diag.proxyLatencyMs ?? '-') + ' ms)',
            'QUIC:      ' + formatDiagBool(diag.quicReachable) + ' (' + (diag.quicLatencyMs ?? '-') + ' ms)',
            'Duration:  ' + diag.timestampMs + ' ms'
        ].join('\n');
    } catch (error) {
        diagnosticsOutput.textContent = 'Diagnostics failed: ' + error;
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
        var profileJson = await tauriInvoke('export_vpn_profile');
        var blob = new Blob([profileJson], { type: 'application/json' });
        var url = URL.createObjectURL(blob);
        var link = document.createElement('a');
        link.href = url;
        link.download = 'freedom-vpn-profile.json';
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
        URL.revokeObjectURL(url);
    } catch (error) {
        alert('Failed to export profile: ' + error);
    }
}

async function importProfile(event) {
    if (!tauriInvoke) {
        alert('Profile import requires the Tauri desktop app runtime.');
        return;
    }

    var file = event.target.files?.[0];
    if (!file) return;

    try {
        var profileJson = await file.text();
        var status = await tauriInvoke('import_vpn_profile', { profileJson: profileJson });
        vpnConnected = !!status.connected;
        applySettingsToForm(status.settings || {});
        updateConnectButton();
        await saveVpnSettings();
    } catch (error) {
        alert('Failed to import profile: ' + error);
    } finally {
        importProfileFile.value = '';
    }
}

/* ── Stats polling ── */
async function fetchNodeStats() {
    try {
        var responses = await Promise.all([
            fetch(NODE_API + '/api/status'),
            fetch(NODE_API + '/api/stats')
        ]);

        if (!responses[0].ok || !responses[1].ok) throw new Error('Bad response');

        var status = await responses[0].json();
        var stats = await responses[1].json();

        document.getElementById('stat-uptime').textContent = formatUptime(status.uptime_ms);
        document.getElementById('stat-active').textContent = status.connections_active;
        document.getElementById('stat-total').textContent = status.connections_total;
        document.getElementById('stat-sent').textContent = formatBytes(stats.bytes_sent);
        document.getElementById('stat-recv').textContent = formatBytes(stats.bytes_received);

        setNodeOnline(true);
    } catch (e) {
        setNodeOnline(vpnConnected);
        STAT_IDS.forEach(function (id) {
            var el = document.getElementById(id);
            if (el) el.textContent = '\u2014';
        });
    }
}

/* ── Event listeners ── */
connectBtn.addEventListener('click', toggleVpnConnection);
saveSettingsBtn.addEventListener('click', saveVpnSettings);
syncSplitBtn.addEventListener('click', syncSplitTunnelApps);
stopSplitBtn.addEventListener('click', stopSplitTunnelApps);
runDiagnosticsBtn.addEventListener('click', runDiagnostics);
exportProfileBtn.addEventListener('click', exportProfile);
importProfileFile.addEventListener('change', importProfile);

/* ── Init ── */
setRuntimeUiState();
loadLocationProfiles();
syncVpnStatus();
fetchNodeStats();
setInterval(fetchNodeStats, 2000);
