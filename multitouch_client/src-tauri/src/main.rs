// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tuio_rs::{Client};
use tauri::Window;
use tuio_rs::client::{CursorEvent, TuioEvents};

// Enum for the different status types -> Create, Update, Delete
#[derive(Clone, serde::Serialize)]
enum Status {
    Create,
    Update,
    Delete,
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    id: i32,
    status: Status,
    coordinates: (f32, f32),
    message: Option<String>,
}

impl Payload {
    fn new(id: i32, status: Status, coordinates: (f32, f32), message: Option<String>) -> Self {
        Self {
            id,
            status,
            coordinates,
            message,
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_background_worker])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn start_background_worker(window: Window) {
    // Start the background worker here
    println!("Starting background worker");

    std::thread::spawn(move || {
        let client = Client::new().unwrap();
        client.connect().expect("Client connecting");

        loop {
            if let Ok(Some(events)) = client.refresh() {
                process_events(events, window.clone());
            }
        }
    });
}

fn process_events(events: TuioEvents, window: Window) {
    for event in events.cursor_events {
        match event {
            CursorEvent::New(data) => {
                let payload = Payload::new(
                    data.cursor.get_session_id(),
                    Status::Create,
                    (data.cursor.get_position().x, data.cursor.get_position().y),
                    None,
                );
                window.emit("finger_update", payload.clone()).unwrap();
            }
            CursorEvent::Update(data) => {
                let payload = Payload::new(
                    data.cursor.get_session_id(),
                    Status::Update,
                    (data.cursor.get_position().x, data.cursor.get_position().y),
                    None,
                );
                window.emit("finger_update", payload.clone()).unwrap();
            }
            CursorEvent::Remove(data) => {
                let payload = Payload::new(
                    data.cursor.get_session_id(),
                    Status::Delete,
                    (data.cursor.get_position().x, data.cursor.get_position().y),
                    None,
                );
                window.emit("finger_update", payload.clone()).unwrap();
            }
        }
    }
}


