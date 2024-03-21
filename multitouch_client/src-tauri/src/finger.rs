use std::sync::{Arc, Mutex, MutexGuard};

use tauri::Window;
use tuio_rs::client::{CursorEvent, TuioEvents};
use crate::button::Button;
use crate::ui::{handle_touch_click, handle_touch_hold};

#[derive(Clone, serde::Serialize, PartialEq)]
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

    fn update(&mut self, coordinates: (f32, f32)) {
        self.coordinates = coordinates;
        self.history.push(coordinates);
        self.status = Status::Update;
    }

    fn delete(&mut self) {
        self.status = Status::Delete;
    }

    fn within_distance_coordinates(&self, coordinates: (f32, f32), distance: f32) -> bool {
        let x = self.coordinates.0 - coordinates.0;
        let y = self.coordinates.1 - coordinates.1;
        (x * x + y * y).sqrt() < distance
    }

    pub(crate) fn get_id(&self) -> i32 {
        self.id
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

pub fn process_finger_event(events: TuioEvents, window: Window, state: Arc<Mutex<(Vec<Finger>, Vec<Button>)>>) {
    for event in events.cursor_events {
        let mut state: MutexGuard<(Vec<Finger>, Vec<Button>)> = state.lock().unwrap();
        let fingers = &mut state.0;
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

                determine_if_hold(finger);
            }
            CursorEvent::Remove(data) => {
                let finger = fingers.iter_mut().find(|f| f.id == data.cursor.get_session_id()).unwrap();
                finger.delete();
                window.emit("finger_update", finger.clone()).unwrap();

                determine_if_click(finger);
            }
        }
    }
}

fn determine_if_hold(finger: &Finger){
    let threshold: usize = 10;
    // if last threshold coordinates are within 0.2 distance of each other, then it's a hold
    if finger.history.len() > threshold {
        let mut hold = true;
        for i in 1..threshold {
            if !finger.within_distance_coordinates(finger.history[finger.history.len() - i], 0.2) {
                hold = false;
                break;
            }
        }
        if hold {
            handle_touch_hold(finger.coordinates, finger);
        }
    }
}

fn determine_if_click(finger: &Finger){
    let threshold: usize = 10;
    // finger.history.len() < threshold then it's a click
    if finger.history.len() < threshold {
        handle_touch_click(finger.coordinates, finger);
    }
}
