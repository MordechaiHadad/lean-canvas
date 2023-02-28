#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{collections::HashMap, fs};

use serde::Serialize;
use tauri::api::{dialog::FileDialogBuilder, file};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_file, save_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn save_file(ids: Vec<String>, values: Vec<String>, name: String) {
    let mut map: HashMap<String, String> = HashMap::new();

    for (key, value) in ids.iter().zip(values.iter()) {
        map.insert(key.to_string(), value.to_string());
    }

    let json = serde_json::to_string(&map).unwrap();

    let (tx, rx) = std::sync::mpsc::channel();

    FileDialogBuilder::new()
        .set_file_name(&name)
        .add_filter("Project File".to_string(), &["txt"])
        .save_file(move |value| {
            tx.send(value).unwrap();
        });

    if let Some(value) = rx.recv().unwrap() {
        fs::write(value, json).unwrap();
    }
}

#[derive(Serialize)]
struct SaveFile {
    name: String,
    data: String
}

#[tauri::command]
fn read_file() -> Option<SaveFile> {
    let (tx, rx) = std::sync::mpsc::channel();

    FileDialogBuilder::new().pick_file(move |value| {
        tx.send(value).unwrap();
    });
    if let Some(value) = rx.recv().unwrap() {
        let data = fs::read_to_string(value.clone()).unwrap();
        let name = value.file_stem().unwrap().to_string_lossy().to_string();
        let save_file = SaveFile { name, data};
        return Some(save_file);
    }
    None
}
