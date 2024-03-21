use std::sync::{Arc, Mutex};
use tauri::Window;
use crate::finger::Finger;

# [derive(Clone, serde::Serialize, Debug)]
pub struct Button {
    id: i32,
    coordinates: (f32, f32),
    dimensions: (f32, f32),
    label: String,
    color: String,
}

impl Button {
    pub fn new(id: i32, coordinates: (f32, f32), dimensions: (f32, f32), label: String, color: String) -> Self {
        Self {
            id,
            coordinates,
            dimensions,
            label,
            color,
        }
    }
}

pub fn is_inside_button(finger: &Finger, ui: (Vec<Finger>, Vec<Button>)) -> Option<Button> {
    let (x, y) = finger.coordinates;
    for button in ui.1.iter() {
        println!("inside button");
        let (button_x, button_y) = button.coordinates;
        let (button_width, button_height) = button.dimensions;
        if x >= button_x && x <= button_x + button_width && y >= button_y && y <= button_y + button_height {
            return Some(button.clone());
        }
    }
    None
}

pub fn send_button_create_event(window: Window, buttons: Vec<Button>) {
    for button in buttons {
        window.emit("button_create", button.clone()).unwrap();
    }
}