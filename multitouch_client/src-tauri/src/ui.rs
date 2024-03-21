use crate::button::{Button, is_inside_button};
use crate::finger::Finger;

# [derive(Clone)]
pub(crate) struct UiStates {
    fingers: Vec<Finger>,
    buttons: Vec<Button>,
}

impl UiStates {
    pub fn new() -> Self {
        println!("Initializing UI");
        Self {
            fingers: Vec::new(),
            buttons: create_buttons(),
        }
    }

    pub fn get_fingers(&mut self) -> &mut Vec<Finger> {
        &mut self.fingers
    }

    pub fn get_buttons(&mut self) -> &mut Vec<Button> {
        &mut self.buttons
    }
}

fn create_buttons() -> Vec<Button> {
    vec![
        Button::new(1, (0.0, 0.0), (0.12, 0.07), "Test".to_string(), "green".to_string()),
    ]
}

pub(crate) fn handle_touch_click(coordinates: (f32, f32), finger: &Finger, ui: &mut UiStates) {
    // Handle touch click here
    println!("Touch click at {:?} by {:?}", coordinates, finger.id);
    let button: Option<Button> = is_inside_button(finger, ui);
    println!("Touch2 click at {:?} by {:?}", coordinates, finger.id);
    if let Some(button) = button{
        println!("Touch1 click at {:?} by {:?}", coordinates, finger.id);
        handle_button_click(button, finger, ui);
    }
    println!("Touch3 click at {:?} by {:?}", coordinates, finger.id);
}

pub(crate) fn handle_touch_hold(coordinates: (f32, f32), finger: &Finger, ui: &mut UiStates) {
    // Handle touch hold here
    // println!("Touch hold at {:?} by {:?}", coordinates, finger.get_id());
}

pub fn handle_button_click(button: Button, finger: &Finger, ui: &mut UiStates) {
    // Handle button click here
    println!("Button click on {:?}", button);
}