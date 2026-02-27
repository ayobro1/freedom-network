// Tab switching functionality
const tabButtons = document.querySelectorAll('.tab-button');
const pages = document.querySelectorAll('.page');
const addressBar = document.getElementById('address-bar');
const statusLocation = document.getElementById('status-location');
const refreshBtn = document.getElementById('refresh-btn');
const goBtn = document.getElementById('go-btn');

const NODE_API = 'http://127.0.0.1:9090';

// Map tabs to pages and URLs
const tabMap = {
    'home': { page: 'home-page', url: 'freedom://home' },
    'chat': { page: 'chat-page', url: 'freedom://chat-site' },
    'example': { page: 'example-page', url: 'freedom://example-site' }
};

// Handle tab button clicks
tabButtons.forEach(button => {
    button.addEventListener('click', () => {
        const tabName = button.getAttribute('data-tab');
        switchTab(tabName);
    });
});

// Handle address bar submission
addressBar.addEventListener('keypress', (e) => {
    if (e.key === 'Enter') {
        navigateTo(addressBar.value);
    }
});

goBtn.addEventListener('click', () => {
    navigateTo(addressBar.value);
});

// Handle refresh button
refreshBtn.addEventListener('click', () => {
    refreshBtn.style.animation = 'spin 0.6s ease-in-out';
    refreshBtn.addEventListener('animationend', () => {
        refreshBtn.style.animation = '';
    }, { once: true });
    fetchNodeStats();
});

// Switch to a tab
function switchTab(tabName) {
    // Update active tab button
    tabButtons.forEach(btn => {
        btn.classList.remove('active');
        if (btn.getAttribute('data-tab') === tabName) {
            btn.classList.add('active');
        }
    });

    // Update active page
    pages.forEach(page => page.classList.remove('active'));
    const pageId = tabMap[tabName].page;
    const activePage = document.getElementById(pageId);
    if (activePage) {
        activePage.classList.add('active');
    }

    // Update address bar and status
    const url = tabMap[tabName].url;
    addressBar.value = '';
    statusLocation.textContent = url;
}

// Navigate to a URL
function navigateTo(url) {
    if (!url.trim()) return;

    // Normalize URL
    const normalizedUrl = url.includes('://') ? url : `freedom://${url}`;
    
    // Determine which tab to switch to
    let tabName = 'home';
    if (normalizedUrl.includes('chat')) {
        tabName = 'chat';
    } else if (normalizedUrl.includes('example')) {
        tabName = 'example';
    }

    // Switch to the appropriate tab
    switchTab(tabName);
    
    // Clear the address bar
    addressBar.value = '';
    addressBar.focus();
}

// Add spin animation for refresh button
const style = document.createElement('style');
style.textContent = `
    @keyframes spin {
        from { transform: rotate(0deg); }
        to { transform: rotate(360deg); }
    }
`;
document.head.appendChild(style);

// Set initial active tab
switchTab('home');

// Listen for address bar changes to update location hint
addressBar.addEventListener('input', (e) => {
    if (e.target.value) {
        const hint = e.target.value.includes('://') 
            ? e.target.value 
            : `freedom://${e.target.value}`;
        statusLocation.textContent = `Going to: ${hint}`;
    } else {
        statusLocation.textContent = 'freedom://home';
    }
});

// --- Real stats from the node API ---

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
    const indicator = document.getElementById('node-indicator');
    const connStatus = document.getElementById('connection-status');
    const statusNode = document.getElementById('status-node');
    if (online) {
        indicator.classList.add('online');
        connStatus.textContent = 'Online';
        statusNode.textContent = '🟢 Node online';
    } else {
        indicator.classList.remove('online');
        connStatus.textContent = 'Offline';
        statusNode.textContent = '🔴 Node offline';
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

console.log('Freedom Browser initialized');
console.log(`Node API: ${NODE_API}`);

