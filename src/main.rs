#[macro_use]
extern crate clap;

use clap::{App, Arg};
use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgba};
use std::cmp::min;

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(Arg::with_name("input file").required(true))
        .arg(Arg::with_name("output file").required(true));

    let matches = app.get_matches();
    let input_file = matches.value_of("input file").unwrap();
    let output_file = matches.value_of("output file").unwrap();

    let image = image::open(input_file).unwrap();
    let (width, height) = image.dimensions();

    let possible_pixel_sizes = (1..=min(width, height))
        .filter(|pixel_size| is_pixel_size_possible(*pixel_size, width, height));

    let image_buffer = possible_pixel_sizes
        .rev()
        .map(|pixel_size| try_export(&image, pixel_size))
        .filter_map(Result::ok)
        .next()
        .unwrap();

    image_buffer.save(output_file).unwrap();
}

fn is_pixel_size_possible(pixel_size: u32, width: u32, height: u32) -> bool {
    (width % pixel_size == 0) && (height % pixel_size == 0)
}

fn get_pixel_value(
    image: &DynamicImage,
    pixel_size: u32,
    original_x: u32,
    original_y: u32,
) -> Result<Rgba<u8>, ()> {
    let mut values = Vec::with_capacity((pixel_size * pixel_size) as usize);

    for offset_x in 0..pixel_size {
        for offset_y in 0..pixel_size {
            values.push(image.get_pixel(original_x + offset_x, original_y + offset_y));
        }
    }

    values.dedup();

    if values.len() > 1 {
        return Err(());
    }

    Ok(values[0])
}

fn try_export(
    image: &DynamicImage,
    pixel_size: u32,
) -> Result<ImageBuffer<Rgba<u8>, Vec<<Rgba<u8> as Pixel>::Subpixel>>, ()> {
    let (original_width, original_height) = image.dimensions();

    let width = original_width / pixel_size;
    let height = original_height / pixel_size;

    let mut image_buffer = ImageBuffer::new(width, height);

    for (original_x, x) in (0..width).map(|x| (x * pixel_size, x)) {
        for (original_y, y) in (0..height).map(|y| (y * pixel_size, y)) {
            get_pixel_value(&image, pixel_size, original_x, original_y).map(|pixel_value| {
                let pixel = image_buffer.get_pixel_mut(x, y);
                *pixel = pixel_value;
            })?
        }
    }

    Ok(image_buffer)
}
