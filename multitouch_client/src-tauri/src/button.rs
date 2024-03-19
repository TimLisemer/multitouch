# [derive(Clone, serde::Serialize)]
pub struct Button {
    id: i32,
    coordinates: (f32, f32),
    dimensions: (f32, f32),
    text: String,
}

impl Button {
    pub fn new(id: i32, coordinates: (f32, f32), dimensions: (f32, f32), text: String) -> Self {
        Self {
            id,
            coordinates,
            dimensions,
            text,
        }
    }
}

pub fn is_inside_button(button: &Button, coordinates: (f32, f32)) -> bool {
    let (x, y) = coordinates;
    let (button_x, button_y) = button.coordinates;
    let (button_width, button_height) = button.dimensions;

    x >= button_x && x <= button_x + button_width && y >= button_y && y <= button_y + button_height
}