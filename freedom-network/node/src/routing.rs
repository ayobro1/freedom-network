// Multi-hop routing and circuit building (onion routing style)

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub struct NodeId([u8; 32]);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub struct CircuitId(pub u32);

#[derive(Debug, Clone)]
pub struct Circuit {
    pub id: CircuitId,
    pub hops: Vec<NodeId>,
    pub current_hop: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CircuitMessage {
    Extend {
        circuit_id: CircuitId,
        next_hop: NodeId,
    },
    Extended {
        circuit_id: CircuitId,
    },
    Data {
        circuit_id: CircuitId,
        encrypted_data: Vec<u8>,
    },
    Destroy {
        circuit_id: CircuitId,
    },
}

pub struct Router {
    circuits: Arc<RwLock<HashMap<CircuitId, Circuit>>>,
    next_circuit_id: Arc<RwLock<u32>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            circuits: Arc::new(RwLock::new(HashMap::new())),
            next_circuit_id: Arc::new(RwLock::new(1)),
        }
    }

    /// Create a new circuit through specified hops
    pub fn build_circuit(&self, hops: Vec<NodeId>) -> CircuitId {
        let mut id_counter = self.next_circuit_id.write().unwrap();
        let circuit_id = CircuitId(*id_counter);
        *id_counter = id_counter.wrapping_add(1);

        let circuit = Circuit {
            id: circuit_id,
            hops,
            current_hop: 0,
        };

        let mut circuits = self.circuits.write().unwrap();
        circuits.insert(circuit_id, circuit);

        circuit_id
    }

    /// Get circuit by ID
    pub fn get_circuit(&self, circuit_id: CircuitId) -> Option<Circuit> {
        let circuits = self.circuits.read().unwrap();
        circuits.get(&circuit_id).cloned()
    }

    /// Advance to next hop in circuit
    pub fn advance_hop(&self, circuit_id: CircuitId) -> bool {
        let mut circuits = self.circuits.write().unwrap();
        if let Some(circuit) = circuits.get_mut(&circuit_id) {
            circuit.current_hop += 1;
            circuit.current_hop < circuit.hops.len()
        } else {
            false
        }
    }

    /// Destroy a circuit
    pub fn destroy_circuit(&self, circuit_id: CircuitId) {
        let mut circuits = self.circuits.write().unwrap();
        circuits.remove(&circuit_id);
    }

    /// Get the next hop node in a circuit
    pub fn get_next_hop(&self, circuit_id: CircuitId) -> Option<NodeId> {
        let circuits = self.circuits.read().unwrap();
        circuits.get(&circuit_id).and_then(|circuit| {
            if circuit.current_hop < circuit.hops.len() {
                Some(circuit.hops[circuit.current_hop])
            } else {
                None
            }
        })
    }

    /// Get exit node (final node) of circuit
    pub fn get_exit_node(&self, circuit_id: CircuitId) -> Option<NodeId> {
        let circuits = self.circuits.read().unwrap();
        circuits.get(&circuit_id).and_then(|circuit| {
            circuit.hops.last().copied()
        })
    }
}