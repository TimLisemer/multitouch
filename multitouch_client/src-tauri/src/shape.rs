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

