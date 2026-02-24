// Freedom Network Client - used by browser to fetch from .freedom sites

use anyhow::Result;
use std::sync::Arc;
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreedomRequest {
    pub domain: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreedomResponse {
    pub status: u16,
    pub headers: std::collections::HashMap<String, String>,
    pub body: Vec<u8>,
}

pub struct FreedomClient {
    resolver: Arc<crate::resolver::FreedomResolver>,
}

impl FreedomClient {
    pub fn new(resolver: Arc<crate::resolver::FreedomResolver>) -> Self {
        Self { resolver }
    }

    /// Fetch content from a .freedom site
    pub async fn fetch(&self, domain: &str, path: &str) -> Result<FreedomResponse> {
        // Resolve the domain
        let metadata = self.resolver.resolve(domain).await?;

        // Build the socket address
        let addr_str = format!("{}:{}", 
            metadata.ipv4.clone().unwrap_or_else(|| "127.0.0.1".to_string()),
            metadata.port
        );
        let _addr: SocketAddr = addr_str.parse()?;

        // For now, return a mock response
        // In the full implementation, this would make a real QUIC connection
        let mut headers = std::collections::HashMap::new();
        headers.insert("Content-Type".to_string(), "text/html".to_string());

        let body = format!(
            "<html><body><h1>Content from {}</h1><p>Path: {}</p></body></html>",
            domain, path
        ).into_bytes();

        Ok(FreedomResponse {
            status: 200,
            headers,
            body,
        })
    }

    /// Fetch with fallback path
    pub async fn fetch_with_index(&self, domain: &str, path: &str) -> Result<FreedomResponse> {
        let clean_path = if path.is_empty() || path == "/" {
            "/index.html".to_string()
        } else {
            path.to_string()
        };

        self.fetch(domain, &clean_path).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let resolver = Arc::new(crate::resolver::FreedomResolver::new(vec![]));
        let _client = FreedomClient::new(resolver);
    }
}
