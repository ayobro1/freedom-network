// Tab switching functionality
const tabButtons = document.querySelectorAll('.tab-button');
const pages = document.querySelectorAll('.page');
const addressBar = document.getElementById('address-bar');
const statusLocation = document.getElementById('status-location');
const refreshBtn = document.getElementById('refresh-btn');
const goBtn = document.getElementById('go-btn');

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

console.log('Freedom Browser initialized');
console.log('Node QUIC server: 127.0.0.1:5000');
