extern crate wasm_bindgen;

use image::DynamicImage;
use image::ImageFormat::PNG;
use wasm_bindgen::prelude::*;

fn handle_export(input: Vec<u8>, pixel_size: Option<u32>) -> Vec<u8> {
    let image = image::load_from_memory(&input).unwrap();

    let image_buffer = if let Some(pixel_size) = pixel_size {
        pixel_perfect_core::force_export(&image, pixel_size)
    } else {
        pixel_perfect_core::export(&image)
    };

    let mut output = Vec::new();
    DynamicImage::ImageRgba8(image_buffer)
        .write_to(&mut output, PNG)
        .unwrap();

    output
}

#[wasm_bindgen]
pub fn export_normally(input: Vec<u8>) -> Vec<u8> {
    handle_export(input, None)
}

#[wasm_bindgen]
pub fn force_export(input: Vec<u8>, pixel_size: u32) -> Vec<u8> {
    handle_export(input, Some(pixel_size))
}
