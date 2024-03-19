use crate::button::Button;
use crate::finger::Finger;

pub fn initialize_ui() -> Vec<Finger> {
    // Initialize the UI here
    println!("Initializing UI");
    let fingers:Vec<Finger> = Vec::new();
    create_buttons();
    fingers
}

fn create_buttons() -> Vec<Button> {
    vec![
        Button::new(1, (0.0, 0.0), (0.2, 0.1), "Test".to_string()),
    ]
}

fn handle_touch_event() {
    // Handle touch events here
    println!("Handling touch event");
}

fn touch_click() {
    // Handle touch click here
    println!("Touch click");
}

fn touch_hold() {
    // Handle touch hold here
    println!("Touch hold");
}