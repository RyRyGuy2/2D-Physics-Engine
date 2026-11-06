use std::vec;

use wasm_bindgen::prelude::*;

use crate::objects::Object;
mod vector;
mod objects;
mod collisions;
// Setup check only; the physics implementation is part of your challenge.
#[wasm_bindgen]
pub fn setup_message() -> String {
    String::from("Rust + WebAssembly connected." )
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello {}!", name)
}

#[wasm_bindgen]
pub fn add_circle(x: f64, y: f64, radius: f64) {
    objects::add_object(objects::Object {
        name: 'c',
        position: vector::Vec2::new(x, y),
        velocity: vector::Vec2::new(0.0, 0.0),
        width: radius * 2.0,
    });
}
