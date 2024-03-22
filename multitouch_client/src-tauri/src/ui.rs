use tauri::{AppHandle, Manager};
use crate::button::{Button, is_inside_button};
use crate::finger::Finger;
use crate::shape::{is_inside_shape, Shape};

# [derive(Clone)]
pub(crate) struct UiStates {
    fingers: Vec<Finger>,
    buttons: Vec<Button>,
    shapes: Vec<Shape>,
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

impl UiStates {
    pub fn new() -> Self {
        println!("Initializing UI");
        Self {
            fingers: Vec::new(),
            buttons: create_buttons(),
            shapes: Vec::new(),
        }
    }
    pub fn get_fingers(&mut self) -> &mut Vec<Finger> {
        &mut self.fingers
    }
    pub fn get_buttons(&mut self) -> &mut Vec<Button> {
        &mut self.buttons
    }
    pub fn get_shapes(&mut self) -> &mut Vec<Shape> {
        &mut self.shapes
    }
}

fn create_buttons() -> Vec<Button> {
    vec![
        Button::new(1, (0.0, 0.0), (0.12, 0.07), "Test".to_string(), "green".to_string()),
    ]
}

pub(crate) fn handle_touch_click(finger: &Finger, ui: &mut UiStates, app_handle: &AppHandle) {
    // Handle touch click here
    let button: Option<Button> = is_inside_button(finger, ui);
    if let Some(button) = button{
        handle_button_click(button.clone(), ui, app_handle);
    }
}

pub(crate) fn handle_touch_hold(finger: &Finger, ui: &mut UiStates, app_handle: &AppHandle) {
    // Handle touch hold here
    let shape: Option<&mut Shape> = is_inside_shape(finger, ui);
    if let Some(shape) = shape {
        handle_shape_hold(shape, finger, app_handle);
    }
}

pub fn handle_button_click(button: Button, ui: &mut UiStates, app_handle: &AppHandle) {
    // Handle button click here
    // let test_rectangle = Shape::new(1, vec![(0.5, 0.5), (0.5, 0.6), (0.6, 0.5), (0.6, 0.6)], 1.0, "blue".to_string());
    println!("\n Button click on {:?}", button);

    let test_rectangle = Shape::new(1, vec![(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0)], 1.0, "blue".to_string());
    ui.get_shapes().push(test_rectangle);

    app_handle.emit_all("button_click", Payload { message: "Tauri is awesome!".into() }).unwrap();
}

pub fn handle_shape_hold(shape: &mut Shape, finger: &Finger, app_handle: &AppHandle) {
    // Handle shape hold here
    println!("\n\nShape hold on {:?}", shape);
    if !shape.concurrent_finger_ids.contains(&finger.id) {
        shape.concurrent_finger_ids.push(finger.id);
    }
}