use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgba};

type RgbaImageBuffer = ImageBuffer<Rgba<u8>, Vec<<Rgba<u8> as Pixel>::Subpixel>>;

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

            red += u32::from(rgba[0]);
            green += u32::from(rgba[1]);
            blue += u32::from(rgba[2]);
            alpha += u32::from(rgba[3]);
        }
    }

    let number_of_pixels = pixel_size * pixel_size;
    red /= number_of_pixels;
    green /= number_of_pixels;
    blue /= number_of_pixels;
    alpha /= number_of_pixels;

    Rgba([red as u8, green as u8, blue as u8, alpha as u8])
}

pub fn force_export(image: &DynamicImage, pixel_size: u32) -> RgbaImageBuffer {
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

pub fn try_export(image: &DynamicImage, pixel_size: u32) -> Result<RgbaImageBuffer, ()> {
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
