// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use tauri::{State, Window};
use tuio_rs::Client;

use crate::button::send_button_create_event;
use crate::finger::process_finger_event;
use crate::ui::UiStates;

mod button;
mod ui;
mod finger;

#[derive(Clone)]
struct MyState {
    ui: Arc<Mutex<UiStates>>,
}

impl MyState {
    fn new(state: UiStates) -> Self {
        Self {
            ui: Arc::new(Mutex::new(state)),
        }
    }
    fn get_ui(&self) -> Arc<Mutex<UiStates>> {
        self.ui.clone()
    }
}

fn main() {
    let ui = UiStates::new();
    let my_state = MyState::new(ui);
    tauri::Builder::default().manage(my_state)
        .invoke_handler(tauri::generate_handler![start_background_worker, button_create])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn start_background_worker(window: Window, state: State<MyState>) {
    // Start the background worker here
    println!("Starting background worker");
    let state_ui = state.get_ui();

    std::thread::spawn(move || {
        let client = Client::new().unwrap();
        client.connect().expect("Client connecting");
        loop {
            if let Ok(Some(events)) = client.refresh() {
                process_finger_event(events, window.clone(), &state_ui.clone());
            }
        }
    });
}


#[tauri::command]
async fn button_create(window: Window, state: State<'_, MyState>) -> Result<(), tauri::Error> {
    println!("Creating buttons");
    let state_ui = state.get_ui();
    send_button_create_event(window.clone(), state_ui.lock().unwrap().get_buttons().clone());
    Ok(())
}



