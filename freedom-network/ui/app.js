const refreshBtn = document.getElementById('refresh-btn');
const statusDot = document.getElementById('status-dot');
const connectionStatus = document.getElementById('connection-status');
const powerRing = document.getElementById('power-ring');
const powerState = document.getElementById('power-state');

const NODE_API = 'http://127.0.0.1:9090';

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
        connectionStatus.textContent = 'Online';
        powerRing.classList.add('online');
        powerState.textContent = 'ONLINE';
    } else {
        statusDot.classList.remove('online');
        connectionStatus.textContent = 'Offline';
        powerRing.classList.remove('online');
        powerState.textContent = 'OFFLINE';
    }
}

const STAT_IDS = ['stat-uptime', 'stat-active', 'stat-total', 'stat-sent', 'stat-recv'];

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
        setNodeOnline(false);
        STAT_IDS.forEach(id => {
            const el = document.getElementById(id);
            if (el) el.textContent = '—';
        });
    }
}

// Poll stats every 2 seconds
fetchNodeStats();
setInterval(fetchNodeStats, 2000);

