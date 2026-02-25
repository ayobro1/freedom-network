#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use fdom::FdomProcessor;
use std::fs;
use std::path::Path;

#[tauri::command]
async fn fetch_freedom_site(domain: String, path: String) -> Result<String, String> {
    // This will fetch from the freedom network node running on 127.0.0.1:5000
    let url = format!("http://127.0.0.1:5000/site/{}/{}", domain, path);
    
    match reqwest::get(&url).await {
        Ok(response) => {
            match response.text().await {
                Ok(content) => {
                    // If it's a .fdom file, render it to HTML
                    if path.ends_with(".fdom") {
                        match FdomProcessor::process(&content) {
                            Ok(html) => Ok(html),
                            Err(e) => Err(format!("Failed to render .fdom: {}", e)),
                        }
                    } else {
                        Ok(content)
                    }
                }
                Err(e) => Err(format!("Failed to read response: {}", e)),
            }
        }
        Err(e) => Err(format!("Failed to fetch from network: {}", e)),
    }
}

#[tauri::command]
fn get_node_status() -> String {
    "Freedom Network Node running on 127.0.0.1:5000".to_string()
}

#[tauri::command]
fn render_fdom(fdom_source: String) -> Result<String, String> {
    FdomProcessor::process(&fdom_source)
        .map_err(|e| format!("Failed to render .fdom: {}", e))
}

#[tauri::command]
fn load_fdom_file(file_path: String) -> Result<String, String> {
    // Load .fdom file from filesystem
    let path = Path::new(&file_path);
    
    if !path.exists() {
        return Err(format!("File not found: {}", file_path));
    }
    
    if !path.extension().and_then(|ext| ext.to_str()).map_or(false, |ext| ext == "fdom") {
        return Err("File must be a .fdom file".to_string());
    }
    
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    FdomProcessor::process(&content)
        .map_err(|e| format!("Failed to render .fdom: {}", e))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            fetch_freedom_site,
            get_node_status,
            render_fdom,
            load_fdom_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
