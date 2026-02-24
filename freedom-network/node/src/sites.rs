// Site server - hosts .freedom sites and serves content

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Site {
    pub domain: String,
    pub content_path: PathBuf,
    pub index_file: String,
}

pub struct SiteServer {
    sites: Arc<RwLock<HashMap<String, Site>>>,
}

impl SiteServer {
    pub fn new() -> Self {
        Self {
            sites: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a site to be hosted
    pub async fn register_site(&self, domain: String, content_path: PathBuf, index_file: String) -> Result<()> {
        let site = Site {
            domain: domain.clone(),
            content_path,
            index_file,
        };
        let mut sites = self.sites.write().await;
        sites.insert(domain, site);
        Ok(())
    }

    /// Get a site by domain
    pub async fn get_site(&self, domain: &str) -> Option<Site> {
        let sites = self.sites.read().await;
        sites.get(domain).cloned()
    }

    /// List all registered sites
    pub async fn list_sites(&self) -> Vec<Site> {
        let sites = self.sites.read().await;
        sites.values().cloned().collect()
    }

    /// Serve content from a site
    pub async fn serve_file(&self, domain: &str, path: &str) -> Result<Vec<u8>> {
        let sites = self.sites.read().await;
        let site = sites.get(domain).ok_or_else(|| anyhow::anyhow!("Site not found: {}", domain))?;

        let file_path = if path.is_empty() || path == "/" {
            site.content_path.join(&site.index_file)
        } else {
            // Remove leading slash
            let clean_path = if path.starts_with('/') { &path[1..] } else { path };
            site.content_path.join(clean_path)
        };

        // Security: ensure path doesn't escape directory
        let canonical = file_path.canonicalize()?;
        let canonical_base = site.content_path.canonicalize()?;

        if !canonical.starts_with(&canonical_base) {
            return Err(anyhow::anyhow!("Path traversal attack detected"));
        }

        Ok(tokio::fs::read(&canonical).await?)
    }

    /// Get site metadata
    pub async fn get_site_info(&self, domain: &str) -> Option<SiteInfo> {
        let sites = self.sites.read().await;
        sites.get(domain).map(|site| SiteInfo {
            domain: site.domain.clone(),
            index_file: site.index_file.clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SiteInfo {
    pub domain: String,
    pub index_file: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[tokio::test]
    async fn test_site_registration() {
        let server = SiteServer::new();
        let temp_dir = tempfile::tempdir().unwrap();

        server.register_site(
            "test.freedom".to_string(),
            temp_dir.path().to_path_buf(),
            "index.html".to_string(),
        ).await.unwrap();

        let site = server.get_site("test.freedom").await;
        assert!(site.is_some());
        assert_eq!(site.unwrap().domain, "test.freedom");
    }

    #[tokio::test]
    async fn test_serve_file() {
        let server = SiteServer::new();
        let temp_dir = tempfile::tempdir().unwrap();
        let index_path = temp_dir.path().join("index.html");
        
        let mut file = std::fs::File::create(&index_path).unwrap();
        file.write_all(b"<html>Hello Freedom</html>").unwrap();

        server.register_site(
            "test.freedom".to_string(),
            temp_dir.path().to_path_buf(),
            "index.html".to_string(),
        ).await.unwrap();

        let content = server.serve_file("test.freedom", "/").await.unwrap();
        assert_eq!(content, b"<html>Hello Freedom</html>");
    }
}
