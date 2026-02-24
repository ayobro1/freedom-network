// Core Freedom Network Protocol
// Handles DHT, routing, and .freedom domain resolution

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Deserialize, Serialize};
use sha3::{Sha3_256, Digest};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NodeId(pub [u8; 32]);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreedomAddress {
    pub domain: String, // e.g., "example.freedom"
    pub node_id: NodeId,
    pub ed25519_pubkey: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentMetadata {
    pub hash: Vec<u8>,
    pub size: u64,
    pub content_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DHTMessage {
    // Find node with closest NodeIds to target
    FindNode {
        target: NodeId,
        requesting_node: NodeId,
    },
    // Store a .freedom domain -> NodeId mapping
    StoreFreedomDomain {
        domain: String,
        owner_node: NodeId,
        pubkey: Vec<u8>,
    },
    // Look up who owns a .freedom domain
    FindFreedomDomain {
        domain: String,
    },
    // Response with peer info
    PeersFound {
        peers: Vec<PeerInfo>,
    },
    // Response with domain owner
    DomainOwner {
        domain: String,
        owner: Option<FreedomAddress>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub node_id: NodeId,
    pub addr: String, // "127.0.0.1:5000"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoutingMessage {
    // Build a multi-hop circuit
    BuildCircuit {
        hops: Vec<NodeId>,
        circuit_id: u32,
    },
    // Route data through circuit
    RelayData {
        circuit_id: u32,
        data: Vec<u8>,
    },
    // Tear down circuit
    DestroyCircuit {
        circuit_id: u32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentMessage {
    // Request content by hash from a .freedom site
    GetContent {
        domain: String,
        path: String,
        circuit_id: u32,
    },
    // Response with content
    ContentData {
        data: Vec<u8>,
        metadata: Option<ContentMetadata>,
    },
    // Not found
    NotFound,
}

// Kademlia-like DHT implementation
pub struct DHT {
    kbuckets: Arc<RwLock<Vec<Vec<PeerInfo>>>>,
    domain_registry: Arc<RwLock<HashMap<String, FreedomAddress>>>,
    content_store: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl DHT {
    pub fn new() -> Self {
        Self {
            kbuckets: Arc::new(RwLock::new(vec![vec![]; 256])),
            domain_registry: Arc::new(RwLock::new(HashMap::new())),
            content_store: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Calculate XOR distance between two node IDs
    pub fn xor_distance(a: &NodeId, b: &NodeId) -> u64 {
        let mut distance = 0u64;
        for i in 0..8 {
            distance ^= u64::from_le_bytes([
                a.0[i],
                a.0[i + 1],
                a.0[i + 2],
                a.0[i + 3],
                a.0[i + 4],
                a.0[i + 5],
                a.0[i + 6],
                a.0[i + 7],
            ]) ^ u64::from_le_bytes([
                b.0[i],
                b.0[i + 1],
                b.0[i + 2],
                b.0[i + 3],
                b.0[i + 4],
                b.0[i + 5],
                b.0[i + 6],
                b.0[i + 7],
            ]);
        }
        distance
    }

    /// Register a .freedom domain
    pub fn register_domain(&self, address: FreedomAddress) -> bool {
        let mut registry = self.domain_registry.write().unwrap();
        registry.insert(address.domain.clone(), address);
        true
    }

    /// Look up a .freedom domain owner
    pub fn lookup_domain(&self, domain: &str) -> Option<FreedomAddress> {
        let registry = self.domain_registry.read().unwrap();
        registry.get(domain).cloned()
    }

    /// Store content (file) keyed by domain
    pub fn store_content(&self, domain: String, content: Vec<u8>) {
        let mut store = self.content_store.write().unwrap();
        store.insert(domain, content);
    }

    /// Retrieve content by domain
    pub fn get_content(&self, domain: &str) -> Option<Vec<u8>> {
        let store = self.content_store.read().unwrap();
        store.get(domain).cloned()
    }

    /// Find peers closest to a target NodeId
    pub fn find_closest_peers(&self, target: &NodeId, k: usize) -> Vec<PeerInfo> {
        let kbuckets = self.kbuckets.read().unwrap();
        let mut all_peers = Vec::new();

        for bucket in kbuckets.iter() {
            all_peers.extend(bucket.clone());
        }

        all_peers.sort_by_key(|peer| Self::xor_distance(&peer.node_id, target));
        all_peers.into_iter().take(k).collect()
    }
}

/// Generate a NodeId from a public key
pub fn generate_node_id(pubkey: &[u8]) -> NodeId {
    let mut hasher = Sha3_256::new();
    hasher.update(pubkey);
    let result = hasher.finalize();
    let mut id = [0u8; 32];
    id.copy_from_slice(&result);
    NodeId(id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_registration() {
        let dht = DHT::new();
        let node_id = NodeId([0u8; 32]);
        let pubkey = vec![1, 2, 3];

        let addr = FreedomAddress {
            domain: "example.freedom".to_string(),
            node_id: node_id.clone(),
            ed25519_pubkey: pubkey,
        };

        assert!(dht.register_domain(addr.clone()));
        assert_eq!(dht.lookup_domain("example.freedom"), Some(addr));
    }

    #[test]
    fn test_xor_distance() {
        let a = NodeId([0x01; 32]);
        let b = NodeId([0x02; 32]);
        let distance = DHT::xor_distance(&a, &b);
        assert!(distance > 0);
    }
}
