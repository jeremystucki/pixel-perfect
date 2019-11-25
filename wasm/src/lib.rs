extern crate wasm_bindgen;

use image::DynamicImage;
use image::ImageFormat::PNG;
use pixel_perfect_core::export;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn handle_input(input: Vec<u8>) -> Vec<u8> {
    let image = image::load_from_memory(&input).unwrap();
    let image_buffer = export(&image);

    let mut output = Vec::new();
    DynamicImage::ImageRgba8(image_buffer)
        .write_to(&mut output, PNG)
        .unwrap();

    output
}
