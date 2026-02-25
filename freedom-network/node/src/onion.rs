// Onion Routing Module for Freedom Network
// Implements multi-hop routing similar to Tor but using our DHT substrate

use rand::seq::SliceRandom;
use sha3::{Digest, Sha3_256};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(String);

#[derive(Clone, Debug)]
pub struct OnionRoute {
    pub route_id: String,
    pub hops: Vec<NodeId>,
    pub symmetric_keys: Vec<Vec<u8>>, // One key per hop
    pub created_at: std::time::SystemTime,
    pub expires_at: std::time::SystemTime,
}

#[derive(Clone, Debug)]
pub struct OnionCircuit {
    pub circuit_id: String,
    pub route: OnionRoute,
    pub state: CircuitState,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CircuitState {
    Building,     // Circuit being established
    Ready,        // Ready for data
    Closing,      // Being torn down
    Closed,       // Closed
}

pub struct OnionRouter {
    circuits: Arc<RwLock<HashMap<String, OnionCircuit>>>,
    available_nodes: Arc<RwLock<Vec<NodeId>>>,
    route_cache: Arc<RwLock<HashMap<String, OnionRoute>>>,
}

impl OnionRouter {
    pub fn new() -> Self {
        OnionRouter {
            circuits: Arc::new(RwLock::new(HashMap::new())),
            available_nodes: Arc::new(RwLock::new(Vec::new())),
            route_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Add a node to the available pool for routing
    pub async fn register_node(&self, node_id: NodeId) {
        let mut nodes = self.available_nodes.write().await;
        if !nodes.contains(&node_id) {
            nodes.push(node_id);
        }
    }
    
    /// Build a multi-hop onion route via available nodes
    pub async fn build_route(&self, num_hops: usize) -> Result<OnionRoute, String> {
        let nodes = self.available_nodes.read().await;
        
        if nodes.len() < num_hops {
            return Err(format!(
                "Not enough nodes available: have {}, need {}",
                nodes.len(),
                num_hops
            ));
        }
        
        // Randomly select hops
        let mut rng = rand::thread_rng();
        let mut selected = nodes
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        selected.shuffle(&mut rng);
        let hops = selected.into_iter().take(num_hops).collect::<Vec<_>>();
        
        // Generate symmetric keys for each hop (for encryption)
        let symmetric_keys: Vec<Vec<u8>> = hops
            .iter()
            .map(|_| self.generate_symmetric_key())
            .collect();
        
        let route_id = self.generate_route_id(&hops);
        let now = std::time::SystemTime::now();
        let expires_at = now
            .checked_add(std::time::Duration::from_secs(3600))
            .unwrap_or(now);
        
        let route = OnionRoute {
            route_id,
            hops,
            symmetric_keys,
            created_at: now,
            expires_at,
        };
        
        Ok(route)
    }
    
    /// Establish an onion circuit
    pub async fn establish_circuit(&self, num_hops: usize) -> Result<String, String> {
        // Build route
        let route = self.build_route(num_hops).await?;
        let circuit_id = self.generate_circuit_id();
        
        // Create circuit
        let circuit = OnionCircuit {
            circuit_id: circuit_id.clone(),
            route: route.clone(),
            state: CircuitState::Building,
        };
        
        // Store circuit
        let mut circuits = self.circuits.write().await;
        circuits.insert(circuit_id.clone(), circuit);
        
        // Cache the route
        let mut cache = self.route_cache.write().await;
        cache.insert(circuit_id.clone(), route);
        
        Ok(circuit_id)
    }
    
    /// Mark circuit as ready to use
    pub async fn activate_circuit(&self, circuit_id: &str) -> Result<(), String> {
        let mut circuits = self.circuits.write().await;
        
        if let Some(circuit) = circuits.get_mut(circuit_id) {
            circuit.state = CircuitState::Ready;
            Ok(())
        } else {
            Err(format!("Circuit {} not found", circuit_id))
        }
    }
    
    /// Get circuit information
    pub async fn get_circuit(&self, circuit_id: &str) -> Option<OnionCircuit> {
        let circuits = self.circuits.read().await;
        circuits.get(circuit_id).cloned()
    }
    
    /// Tear down a circuit
    pub async fn close_circuit(&self, circuit_id: &str) -> Result<(), String> {
        let mut circuits = self.circuits.write().await;
        
        if let Some(circuit) = circuits.get_mut(circuit_id) {
            circuit.state = CircuitState::Closing;
            // Async cleanup would happen here
            circuit.state = CircuitState::Closed;
            Ok(())
        } else {
            Err(format!("Circuit {} not found", circuit_id))
        }
    }
    
    /// Layer encrypt data through all hops (onion-style)
    pub fn encrypt_payload(
        &self,
        payload: &[u8],
        symmetric_keys: &[Vec<u8>],
    ) -> Vec<u8> {
        let mut encrypted = payload.to_vec();
        
        // Encrypt in reverse order (exit node first, entry node last)
        for key in symmetric_keys.iter().rev() {
            encrypted = self.xor_encrypt(&encrypted, key);
        }
        
        encrypted
    }
    
    /// Layer decrypt data through all hops
    pub fn decrypt_payload(
        &self,
        encrypted: &[u8],
        symmetric_keys: &[Vec<u8>],
    ) -> Vec<u8> {
        let mut decrypted = encrypted.to_vec();
        
        // Decrypt in forward order (entry node first, exit node last)
        for key in symmetric_keys.iter() {
            decrypted = self.xor_encrypt(&decrypted, key);
        }
        
        decrypted
    }
    
    // ===== Private Helper Methods =====
    
    fn generate_symmetric_key(&self) -> Vec<u8> {
        use sha3::Digest;
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        
        let mut hasher = Sha3_256::new();
        hasher.update(format!("{}", timestamp).as_bytes());
        hasher.finalize().to_vec()
    }
    
    fn generate_route_id(&self, hops: &[NodeId]) -> String {
        let mut hasher = Sha3_256::new();
        for hop in hops {
            hasher.update(hop.0.as_bytes());
        }
        hex::encode(hasher.finalize())
    }
    
    fn generate_circuit_id(&self) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let random_bytes: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
        hex::encode(random_bytes)
    }
    
    fn xor_encrypt(&self, data: &[u8], key: &[u8]) -> Vec<u8> {
        data.iter()
            .zip(key.iter().cycle())
            .map(|(d, k)| d ^ k)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_build_route() {
        let router = OnionRouter::new();
        
        // Register some nodes
        router.register_node(NodeId("node1".to_string())).await;
        router.register_node(NodeId("node2".to_string())).await;
        router.register_node(NodeId("node3".to_string())).await;
        
        // Build a 3-hop route
        let route = router.build_route(3).await.unwrap();
        
        assert_eq!(route.hops.len(), 3);
        assert_eq!(route.symmetric_keys.len(), 3);
        assert!(!route.route_id.is_empty());
    }
    
    #[tokio::test]
    async fn test_establish_circuit() {
        let router = OnionRouter::new();
        
        router.register_node(NodeId("node1".to_string())).await;
        router.register_node(NodeId("node2".to_string())).await;
        router.register_node(NodeId("node3".to_string())).await;
        
        let circuit_id = router.establish_circuit(3).await.unwrap();
        assert!(!circuit_id.is_empty());
        
        let circuit = router.get_circuit(&circuit_id).await.unwrap();
        assert_eq!(circuit.state, CircuitState::Building);
    }
    
    #[tokio::test]
    async fn test_encrypt_decrypt() {
        let router = OnionRouter::new();
        
        let payload = b"Secret message";
        let keys = vec![
            vec![0x42; 32],
            vec![0x99; 32],
            vec![0xAA; 32],
        ];
        
        let encrypted = router.encrypt_payload(payload, &keys);
        let decrypted = router.decrypt_payload(&encrypted, &keys);
        
        assert_eq!(decrypted, payload);
    }
}
