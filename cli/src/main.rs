#[macro_use]
extern crate clap;

use clap::{App, Arg};
use image::GenericImageView;
use pixel_perfect_core::{force_export, try_export};
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
                .long("--force-pixel-size")
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
