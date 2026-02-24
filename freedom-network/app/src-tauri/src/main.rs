#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

#[tauri::command]
async fn fetch_freedom_site(domain: String, path: String) -> Result<String, String> {
    // This will fetch from the freedom network node running on 127.0.0.1:5000
    let url = format!("http://127.0.0.1:5000/site/{}/{}", domain, path);
    
    match reqwest::get(&url).await {
        Ok(response) => {
            match response.text().await {
                Ok(content) => Ok(content),
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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_freedom_site, get_node_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
