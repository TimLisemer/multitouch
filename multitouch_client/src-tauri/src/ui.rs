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
        Button::new(1, (0.0, 0.0), (0.12, 0.07), "Test".to_string(), "green".to_string(), "blue".to_string()),
    ]
}

pub(crate) fn handle_touch_click(finger: &Finger, ui: &mut UiStates, app_handle: &AppHandle) {
    // Handle touch click here
    let button: Option<Button> = is_inside_button(finger, ui);
    if let Some(button) = button{
        handle_button_click(button.clone(), ui, app_handle, finger.id);
    }
}

pub(crate) fn handle_touch_hold(finger: &Finger, ui: &mut UiStates, app_handle: &AppHandle) {
    // Handle touch hold here
    for shape in ui.get_shapes().iter_mut() {
        if shape.concurrent_finger_ids.contains(&finger.id) {
            handle_shape_hold(shape, finger, app_handle);
            return;
        }
    }

    let shape: Option<&mut Shape> = is_inside_shape(finger, ui);
    if let Some(shape) = shape {
        handle_shape_hold(shape, finger, app_handle);
    }
}

pub fn handle_button_click(button: Button, ui: &mut UiStates, app_handle: &AppHandle, finger_id: i32) {
    // Handle button click here
    let mut button: &mut Button = ui.get_buttons().iter_mut().find(|b| b.id == button.id).unwrap();
    println!("Button click on {:?}", button);
    button.fingers.push(finger_id);

    let original_color = button.color.clone();
    let original_mode_color = button.mode_color.clone();
    button.color = original_mode_color;
    button.mode_color = original_color;
    button.mode = !button.mode;
    app_handle.emit_all("update_button_color", button.clone()).unwrap();

    let vertices = vec![
        (0.3, 0.4),
        (0.7, 0.4),
        (0.7, 0.6),
        (0.3, 0.6),
    ];

    let test_rectangle = Shape::new(ui.get_shapes().len() as i32, vertices, 1.0);
    ui.get_shapes().push(test_rectangle.clone());

    app_handle.emit_all("create_shape", test_rectangle.clone()).unwrap();
}

pub fn handle_shape_hold(shape: &mut Shape, finger: &Finger, app_handle: &AppHandle) {
    // Handle shape hold here
    if !shape.concurrent_finger_ids.contains(&finger.id) {
        shape.concurrent_finger_ids.push(finger.id);
    }
    shape.move_shape(finger.clone());
    app_handle.emit_all("update_shape", shape.clone()).unwrap();
}