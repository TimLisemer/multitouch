// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread::sleep;
use tauri::Window;

// Enum for the different status types -> Create, Update, Delete
#[derive(Clone, serde::Serialize)]
enum Status {
    Create,
    Update,
    Delete,
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    id: u32,
    status: Status,
    coordinates: (f32, f32),
    message: String,
}

#[tauri::command]
fn start_background_worker(window: Window) {
    // Start the background worker here
    println!("Starting background worker");

    // Test Paylaoad Data
    let id = 15;
    let status = Status::Create;
    let coordinates = (50.0, 100.0);
    let message = "Test message".to_string();

    let payload = Payload {id, status, coordinates, message};

    std::thread::spawn(move || {
        loop {
            window.emit("finger_update", payload.clone()).unwrap();
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
