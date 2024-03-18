// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread::sleep;
use tauri::Window;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[tauri::command]
fn start_background_worker(window: Window) {
    // Start the background worker here
    println!("Starting background worker");
    std::thread::spawn(move || {
        loop {
            window.emit("finger_update", Payload { message: "Tauri is awesome!".into() }).unwrap();
            sleep(std::time::Duration::from_secs(1));
        }
    });
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_background_worker])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
