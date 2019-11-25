extern crate wasm_bindgen;

use image::GenericImageView;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn handle_input(input: Vec<u8>) {
    let image = image::load_from_memory(&input).unwrap();
    alert(&format!(
        "image is {} x {}",
        image.dimensions().0,
        image.dimensions().1
    ));
}
