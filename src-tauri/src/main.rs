#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{collections::HashMap, fs};

use serde::Serialize;
use tauri::api::dialog::FileDialogBuilder;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_file, save_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn save_file(ids: Vec<String>, values: Vec<String>, name: String, open_dialogue: bool) -> bool {
    let json = produce_json(&ids, &values);

    let (tx, rx) = std::sync::mpsc::channel();

    if fs::metadata(format!("{name}.txt")).is_ok() && !open_dialogue {
        fs::write(format!("{name}.txt"), json.clone()).unwrap();
        return true; 
    }
    

    FileDialogBuilder::new()
        .set_file_name(&name)
        .add_filter("Project File".to_string(), &["txt"])
        .save_file(move |value| {
            tx.send(value).unwrap();
        });

    if let Some(value) = rx.recv().unwrap() {
        fs::write(value, json).unwrap();
        return true;
    }

    return false;
}


#[derive(Serialize)]
struct DocumentFile {
    name: String,
    data: String,
}

fn produce_json(ids: &Vec<String>, values: &Vec<String>) -> String {
    let mut map: HashMap<String, String> = HashMap::new();

    for (key, value) in ids.iter().zip(values.iter()) {
        map.insert(key.to_string(), value.to_string());
    }
    serde_json::to_string(&map).unwrap()
}

#[tauri::command]
fn read_file() -> Option<DocumentFile> {
    let (tx, rx) = std::sync::mpsc::channel();

    FileDialogBuilder::new()
        .add_filter("Project File", &["txt"])
        .pick_file(move |value| {
            tx.send(value).unwrap();
        });
    if let Some(value) = rx.recv().unwrap() {
        let data = fs::read_to_string(value.clone()).unwrap();
        let name = value.file_stem().unwrap().to_string_lossy().to_string();
        let save_file = DocumentFile { name, data };
        return Some(save_file);
    }
    None
}
