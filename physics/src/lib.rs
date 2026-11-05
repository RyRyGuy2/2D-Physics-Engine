use wasm_bindgen::prelude::*;

// Setup check only; the physics implementation is part of your challenge.
#[wasm_bindgen]
pub fn setup_message() -> String {
    String::from("Rust + WebAssembly connected.")
}
