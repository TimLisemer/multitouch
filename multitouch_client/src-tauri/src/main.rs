// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod button;
mod ui;
mod finger;

use std::sync::{Arc, Mutex};
use tuio_rs::{Client};
use tauri::{State, Window};
use crate::ui::initialize_ui;
use crate::finger::{Finger, process_finger_event};

#[derive(Clone)]
struct MyState {
    ui: Arc<Mutex<Vec<Finger>>>,
}

impl MyState {
    fn new(fingers: Vec<Finger>) -> Self {
        Self {
            ui: Arc::new(Mutex::new(fingers)),
        }
    }
    fn get_ui(&self) -> Arc<Mutex<Vec<Finger>>> {
        self.ui.clone()
    }
}

fn main() {
    let ui = initialize_ui();
    let my_state = MyState::new(ui);
    tauri::Builder::default().manage(my_state)
        .invoke_handler(tauri::generate_handler![start_background_worker])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn start_background_worker(window: Window, state: State<MyState>) {
    // Start the background worker here
    println!("Starting background worker");
    let state_cone = state.get_ui();

    std::thread::spawn(move || {
        let client = Client::new().unwrap();
        client.connect().expect("Client connecting");

        loop {
            if let Ok(Some(events)) = client.refresh() {
                process_finger_event(events, window.clone(), state_cone.clone());
            }
        }
    });
}


