use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;

/// Simple peer manager
#[derive(Clone)]
pub struct PeerManager {
    peers: Arc<Mutex<HashSet<SocketAddr>>>,
}

impl PeerManager {
    /// Create a new peer manager
    pub fn new() -> Self {
        PeerManager {
            peers: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    /// Add a peer
    pub fn add_peer(&self, addr: SocketAddr) {
        let mut peers = self.peers.lock().unwrap();
        peers.insert(addr);
    }

    /// Remove a peer
    pub fn remove_peer(&self, addr: &SocketAddr) {
        let mut peers = self.peers.lock().unwrap();
        peers.remove(addr);
    }

    /// Get list of current peers
    pub fn list_peers(&self) -> Vec<SocketAddr> {
        let peers = self.peers.lock().unwrap();
        peers.iter().cloned().collect()
    }
}

/// Log messages with a timestamp
pub fn log(msg: &str) {
    let now = chrono::Utc::now();
    println!("[{}] {}", now.format("%Y-%m-%d %H:%M:%S"), msg);
}

/// Format a peer address as string
pub fn format_peer(addr: &SocketAddr) -> String {
    addr.to_string()
}