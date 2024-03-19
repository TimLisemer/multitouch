use std::sync::{Arc, Mutex, MutexGuard};

use tauri::Window;
use tuio_rs::client::{CursorEvent, TuioEvents};

#[derive(Clone, serde::Serialize)]
pub(crate) enum Status {
    Create,
    Update,
    Delete,
}

# [derive(Clone, serde::Serialize)]
pub struct Finger {
    id: i32,
    coordinates: (f32, f32),
    history: Vec<(f32, f32)>,
    color: String,
    status: Status,
}

impl Finger {
    pub fn new(id: i32, coordinates: (f32, f32)) -> Self {
        Self {
            id,
            coordinates,
            history: vec![coordinates],
            color: get_random_color(),
            status: Status::Create,
        }
    }

    pub fn update(&mut self, coordinates: (f32, f32)) {
        self.coordinates = coordinates;
        self.history.push(coordinates);
        self.status = Status::Update;
    }

    pub fn delete(&mut self) {
        self.status = Status::Delete;
    }
}

fn get_random_color() -> String {
    let letters = "0123456789ABCDEF";
    let mut color = "#".to_string();
    for _ in 0..6 {
        color.push(letters.chars().nth(rand::random::<usize>() % 16).unwrap());
    }
    color
}

pub fn process_finger_event(events: TuioEvents, window: Window, state: Arc<Mutex<Vec<Finger>>>) {
    for event in events.cursor_events {
        let mut fingers: MutexGuard<Vec<Finger>> = state.lock().unwrap();
        match event {
            CursorEvent::New(data) => {
                let finger = Finger::new(data.cursor.get_session_id(), (data.cursor.get_position().x, data.cursor.get_position().y));
                fingers.push(finger.clone());
                window.emit("finger_update", finger.clone()).unwrap();
            }
            CursorEvent::Update(data) => {
                let finger = fingers.iter_mut().find(|f| f.id == data.cursor.get_session_id()).unwrap();
                finger.update((data.cursor.get_position().x, data.cursor.get_position().y));
                window.emit("finger_update", finger.clone()).unwrap();
            }
            CursorEvent::Remove(data) => {
                let finger = fingers.iter_mut().find(|f| f.id == data.cursor.get_session_id()).unwrap();
                finger.delete();
                window.emit("finger_update", finger.clone()).unwrap();
            }
        }
    }
}
