// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to Nebula Terminal!", name)
}

#[tauri::command]
fn get_terminal_info() -> serde_json::Value {
    serde_json::json!({
        "name": "Nebula Terminal",
        "version": "0.1.0",
        "status": "ready"
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_terminal_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 