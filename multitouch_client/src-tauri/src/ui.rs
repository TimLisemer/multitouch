use crate::button::{Button, is_inside_button};
use crate::finger::Finger;

pub fn initialize_ui() -> (Vec<Finger>, Vec<Button>) {
    // Initialize the UI here
    println!("Initializing UI");
    let fingers:Vec<Finger> = Vec::new();
    let buttons: Vec<Button> = create_buttons();
    let state: (Vec<Finger>, Vec<Button>) = (fingers, buttons);

    state
}

fn create_buttons() -> Vec<Button> {
    vec![
        Button::new(1, (0.0, 0.0), (0.12, 0.07), "Test".to_string(), "green".to_string()),
    ]
}

pub(crate) fn handle_touch_click(coordinates: (f32, f32), finger: &Finger, ui: &(Vec<Finger>, Vec<Button>)) {
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

pub(crate) fn handle_touch_hold(coordinates: (f32, f32), finger: &Finger, ui: &(Vec<Finger>, Vec<Button>)) {
    // Handle touch hold here
    // println!("Touch hold at {:?} by {:?}", coordinates, finger.get_id());
}

pub fn handle_button_click(button: Button, finger: &Finger, ui: &(Vec<Finger>, Vec<Button>)) {
    // Handle button click here
    println!("Button click on {:?}", button);
}