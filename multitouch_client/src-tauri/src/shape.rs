use tauri::Window;
use crate::finger::{Finger, get_random_color};
use crate::ui::UiStates;

# [derive(Clone, serde::Serialize, Debug)]
pub struct Shape {
    id: i32,
    vertices: Vec<(f32, f32)>,
    scale: f32,
    color: String,
    pub concurrent_finger_ids: Vec<i32>,
}

impl Shape {
    pub fn new(id: i32, vertices: Vec<(f32, f32)>, scale: f32) -> Self {
        Self {
            id,
            vertices,
            scale,
            color: get_random_color(),
            concurrent_finger_ids: Vec::new(),
        }
    }

    pub fn move_shape(&mut self, finger: Finger) {
        let (click_x, click_y) = finger.coordinates;
        let (center_x, center_y) = calculate_center(&self.vertices);

        // Calculate the difference between the clicked point and the center of the shape
        let dx = click_x - center_x;
        let dy = click_y - center_y;

        // Iterate over vertices and move each one by the same difference
        for vertex in self.vertices.iter_mut() {
            vertex.0 += dx;
            vertex.1 += dy;
        }
    }

}

pub fn is_inside_shape<'a>(finger: &Finger, ui: &'a mut UiStates) -> Option<&'a mut Shape> {
    let (x, y) = finger.coordinates;
    for shape in ui.get_shapes().iter_mut() {
        let mut intersections = 0;
        let vertices = &shape.vertices;
        let len = vertices.len();
        for i in 0..len {
            let (x1, y1) = vertices[i];
            let (x2, y2) = vertices[(i + 1) % len];
            if ((y1 > y) != (y2 > y)) && (x < (x2 - x1) * (y - y1) / (y2 - y1) + x1) {
                intersections += 1;
            }
        }
        if intersections % 2 == 1 {
            return Some(shape);
        }
    }
    None
}

pub fn send_object_create_event(window: Window, objects: Vec<Shape>) {
    for object in objects {
        window.emit("object_create", object.clone()).unwrap();
    }
}

pub fn calculate_center(vertices: &Vec<(f32, f32)>) -> (f32, f32) {
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let len = vertices.len() as f32;

    for &(x, y) in vertices {
        sum_x += x;
        sum_y += y;
    }

    (sum_x / len, sum_y / len)
}
