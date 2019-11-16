#[macro_use]
extern crate clap;

use clap::{App, Arg};
use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgba};
use std::cmp::min;
use std::str::FromStr;

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(Arg::with_name("input file").required(true))
        .arg(Arg::with_name("output file").required(true))
        .arg(
            Arg::with_name("pixel size")
                .long("--pixel-size")
                .takes_value(true),
        );

    let matches = app.get_matches();

    let input_file = matches.value_of("input file").unwrap();
    let output_file = matches.value_of("output file").unwrap();
    let pixel_size = matches
        .value_of("pixel size")
        .map(|pixel_size| u32::from_str(pixel_size).expect("pixel size must be a number"));

    let image = image::open(input_file).unwrap();

    let image_buffer = if let Some(pixel_size) = pixel_size {
        force_export(&image, pixel_size)
    } else {
        let (width, height) = image.dimensions();

        let possible_pixel_sizes = (1..=min(width, height))
            .filter(|pixel_size| is_pixel_size_possible(*pixel_size, width, height));

        possible_pixel_sizes
            .rev()
            .map(|pixel_size| try_export(&image, pixel_size))
            .filter_map(Result::ok)
            .next()
            .unwrap()
    };

    image_buffer.save(output_file).unwrap();
}

fn is_pixel_size_possible(pixel_size: u32, width: u32, height: u32) -> bool {
    (width % pixel_size == 0) && (height % pixel_size == 0)
}

fn get_average_pixel_value(
    image: &DynamicImage,
    pixel_size: u32,
    original_x: u32,
    original_y: u32,
) -> Rgba<u8> {
    let mut red: u32 = 0;
    let mut green: u32 = 0;
    let mut blue: u32 = 0;
    let mut alpha: u32 = 0;

    for offset_x in 0..pixel_size {
        for offset_y in 0..pixel_size {
            let pixel = image.get_pixel(original_x + offset_x, original_y + offset_y);
            let rgba = pixel.0;

            red += rgba[0] as u32;
            green += rgba[1] as u32;
            blue += rgba[2] as u32;
            alpha += rgba[3] as u32;
        }
    }

    let number_of_pixels = pixel_size * pixel_size;
    red /= number_of_pixels;
    green /= number_of_pixels;
    blue /= number_of_pixels;
    alpha /= number_of_pixels;

    Rgba([red as u8, green as u8, blue as u8, alpha as u8])
}

fn force_export(
    image: &DynamicImage,
    pixel_size: u32,
) -> ImageBuffer<Rgba<u8>, Vec<<Rgba<u8> as Pixel>::Subpixel>> {
    let (original_width, original_height) = image.dimensions();

    let width = original_width / pixel_size;
    let height = original_height / pixel_size;

    let mut image_buffer = ImageBuffer::new(width, height);

    for (original_x, x) in (0..width).map(|x| (x * pixel_size, x)) {
        for (original_y, y) in (0..height).map(|y| (y * pixel_size, y)) {
            let pixel = image_buffer.get_pixel_mut(x, y);
            *pixel = get_average_pixel_value(&image, pixel_size, original_x, original_y);
        }
    }

    image_buffer
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
