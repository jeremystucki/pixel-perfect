use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use std::cmp::min;

fn main() {
    let image = image::open("input.png").unwrap();
    let (width, height) = image.dimensions();

    let mut possible_pixel_sizes: Vec<_> = (1..=min(width, height))
        .filter(|pixel_size| is_pixel_size_possible(*pixel_size, width, height))
        .collect();

    possible_pixel_sizes.reverse();

    for pixel_size in possible_pixel_sizes {
        if let Ok(_) = try_export(&image, pixel_size) {
            break;
        }
    }
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

fn try_export(image: &DynamicImage, pixel_size: u32) -> Result<(), ()> {
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

    image_buffer.save("output.png").unwrap();

    Ok(())
}
