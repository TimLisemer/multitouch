use std::sync::{Arc, Mutex, MutexGuard};

use tauri::{AppHandle, Manager, Window};
use tuio_rs::client::{CursorEvent, TuioEvents};
use crate::button::Button;

use crate::ui::{handle_touch_click, handle_touch_hold, UiStates};

#[derive(Clone, serde::Serialize, PartialEq, Debug)]
pub(crate) enum Status {
    Create,
    Update,
    Delete,
}

# [derive(Clone, serde::Serialize, Debug)]
pub struct Finger {
    pub(crate) id: i32,
    pub(crate) coordinates: (f32, f32),
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
}

pub(crate) fn get_random_color() -> String {
    let letters = "0123456789ABCDEF";
    let mut color = "#".to_string();
    for _ in 0..6 {
        color.push(letters.chars().nth(rand::random::<usize>() % 16).unwrap());
    }
    color
}

pub fn process_finger_event(events: TuioEvents, window: Window, state: &Arc<Mutex<UiStates>>, app_handle: AppHandle) {
    // let state_ui_clone = state.lock().unwrap().clone();
    let mut state_ui: MutexGuard<UiStates> = state.lock().unwrap();
    for event in events.cursor_events {
        let fingers = &mut state_ui.get_fingers();
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

                determine_if_hold(finger.clone(), &mut *state_ui, &app_handle);
            }
            CursorEvent::Remove(data) => {
                let finger = fingers.iter_mut().find(|f| f.id == data.cursor.get_session_id()).unwrap();
                finger.delete();
                window.emit("finger_update", finger.clone()).unwrap();

                handle_remove_finger(finger.clone(), &mut *state_ui, &app_handle);
            }
        }
    }
}

fn determine_if_hold(finger: Finger, mut ui: &mut UiStates, app_handle: &AppHandle){
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
            handle_touch_hold(&finger, &mut ui, app_handle);
        }
    }
}

fn handle_remove_finger(finger: Finger, ui: &mut UiStates, app_handle: &AppHandle){
    for shape in ui.get_shapes().iter_mut() {
        if shape.concurrent_finger_ids.contains(&finger.id) {
            shape.concurrent_finger_ids.retain(|&x| x != finger.id);
            break;
        }
    }

    determine_if_click(finger.clone(), &mut *ui, app_handle);

    // Check if button is in mode
    let mut button: Option<&mut Button> = ui.get_buttons().get_mut(0);
    if let Some(button_ref) = button.as_mut() {
        if button_ref.mode && !button_ref.fingers.contains(&finger.id){
            app_handle.emit_all("detect_shape", finger.clone()).unwrap();
            button_ref.mode = false; // Mutate the mode field directly through the mutable reference
            button_ref.fingers.clear();
            println!("Button mode off");
        }
    }
}

fn determine_if_click(finger: Finger, mut ui: &mut UiStates, app_handle: &AppHandle){
    let threshold: usize = 10;
    // finger.history.len() < threshold then it's a click
    if finger.history.len() < threshold {
        handle_touch_click(&finger, &mut ui, app_handle);
    }
}