use crate::vector;
use std::cell::RefCell;

thread_local! {
    pub static OBJECTS: RefCell<Vec<Object>> = const { RefCell::new(Vec::new()) };
}

// width is diameter for circles
#[derive(Copy, Clone)]
pub struct Object {
    pub name: char, // c or s
    pub position: vector::Vec2,
    pub velocity: vector::Vec2, // velocity in px / s (irl 1px = 0.1m)
    pub width: f64,
}

/// Adds an object and returns its index in OBJECTS.
pub fn add_object(object: Object) -> usize {
    OBJECTS.with(|objects| {
        let mut objects = objects.borrow_mut();
        let index = objects.len();
        objects.push(object);
        index
    })
}

/// Replaces an object's values. Returns false if the index does not exist.
pub fn update_object(index: usize, object: Object) -> bool {
    OBJECTS.with(|objects| {
        let mut objects = objects.borrow_mut();
        match objects.get_mut(index) {
            Some(existing) => {
                *existing = object;
                true
            }
            None => false,
        }
    })
}
