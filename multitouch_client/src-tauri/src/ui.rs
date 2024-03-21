use crate::button::Button;
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

pub(crate) fn handle_touch_click(coordinates: (f32, f32), finger: &Finger) {
    // Handle touch click here
    println!("Touch click at {:?} by {:?}", coordinates, finger.get_id());
}

pub(crate) fn handle_touch_hold(coordinates: (f32, f32), finger: &Finger) {
    // Handle touch hold here
    println!("Touch hold at {:?} by {:?}", coordinates, finger.get_id());
}