// .freedom resolver - finds and connects to .freedom sites

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreedomSiteMetadata {
    pub domain: String,
    pub owner_node_id: Vec<u8>,
    pub ipv4: Option<String>,
    pub ipv6: Option<String>,
    pub port: u16,
    pub protocol_version: u32,
}

pub struct FreedomResolver {
    cache: Arc<RwLock<HashMap<String, FreedomSiteMetadata>>>,
    bootstrap_nodes: Vec<String>,
}

impl FreedomResolver {
    pub fn new(bootstrap_nodes: Vec<String>) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            bootstrap_nodes,
        }
    }

    /// Resolve a .freedom domain to site metadata
    pub async fn resolve(&self, domain: &str) -> Result<FreedomSiteMetadata> {
        // Normalize domain
        let domain = if domain.ends_with(".freedom") {
            domain.to_string()
        } else {
            format!("{}.freedom", domain)
        };

        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(metadata) = cache.get(&domain) {
                return Ok(metadata.clone());
            }
        }

        // Try to resolve through bootstrap nodes
        // For now, return a mock response
        let metadata = FreedomSiteMetadata {
            domain: domain.clone(),
            owner_node_id: vec![0x01; 32],
            ipv4: Some("127.0.0.1".to_string()),
            ipv6: None,
            port: 5000,
            protocol_version: 1,
        };

        // Cache the result
        {
            let mut cache = self.cache.write().await;
            cache.insert(domain, metadata.clone());
        }

        Ok(metadata)
    }

    /// Clear the resolution cache
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }

    /// Get cached domains
    pub async fn list_cached(&self) -> Vec<String> {
        let cache = self.cache.read().await;
        cache.keys().cloned().collect()
    }

    /// Add custom domain mapping (for local testing)
    pub async fn add_mapping(&self, domain: String, metadata: FreedomSiteMetadata) {
        let mut cache = self.cache.write().await;
        cache.insert(domain, metadata);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_domain_resolution() {
        let resolver = FreedomResolver::new(vec!["127.0.0.1:5000".to_string()]);
        let result = resolver.resolve("example").await;
        
        assert!(result.is_ok());
        let metadata = result.unwrap();
        assert_eq!(metadata.domain, "example.freedom");
        assert_eq!(metadata.port, 5000);
    }

    #[tokio::test]
    async fn test_cache() {
        let resolver = FreedomResolver::new(vec![]);
        let domain = "test".to_string();

        // Add to cache
        let metadata = FreedomSiteMetadata {
            domain: "test.freedom".to_string(),
            owner_node_id: vec![0x02; 32],
            ipv4: Some("192.168.1.1".to_string()),
            ipv6: None,
            port: 8000,
            protocol_version: 1,
        };
        resolver.add_mapping(domain, metadata).await;

        // Check it's cached
        let cached = resolver.list_cached().await;
        assert_eq!(cached.len(), 1);
    }
}
