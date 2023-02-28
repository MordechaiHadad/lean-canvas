#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{collections::HashMap, fs};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn test_command(ids: Vec<String>, values: Vec<String>) {
    let mut map: HashMap<String, String> = HashMap::new();

    for (key, value) in ids.iter().zip(values.iter()) {
        map.insert(key.to_string(), value.to_string());
    }

    let json = serde_json::to_string(&map).unwrap();

    fs::write("test.txt", json).unwrap();
}

#[tauri::command]
fn load_file() {}
