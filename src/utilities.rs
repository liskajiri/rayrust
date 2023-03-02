use std::fs::OpenOptions;
use std::io::Write;

use image::{Rgb, RgbImage};
use rand::Rng;

use crate::vec3::Color;

pub fn write_buffer_to_file(
    filepath: &String,
    buffer: &Vec<Color>,
    samples_per_pixel: u32,
    image_width: u32,
    image_height: u32,
) {
    fn clamp_color(color: f64) -> i32 {
        (256.0 * clamp(color, 0.0, 0.999)) as i32
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(filepath)
        .unwrap();

    // Image
    let mut ppm_image = format!("P3\n{image_width} {image_height}\n255\n");

    let scale = 1.0 / (samples_per_pixel as f64);

    for pixel_color in buffer {
        let scaled_color = *pixel_color * scale;
        let r = scaled_color.x.sqrt();
        let g = scaled_color.y.sqrt();
        let b = scaled_color.z.sqrt();
        let formatted_color =
            format!("{} {} {}\n", clamp_color(r), clamp_color(g), clamp_color(b),);
        ppm_image.push_str(&formatted_color);
    }
    file.write_all(ppm_image.as_bytes()).unwrap();
}

pub fn _save_as_png(
    filename: &str,
    buffer: &Vec<Color>,
    samples_per_pixel: u32,
    image_width: u32,
    image_height: u32,
) {
    fn clamp_color(color: f64) -> u8 {
        (256.0 * clamp(color, 0.0, 0.999)) as u8
    }

    let mut rgb_image = RgbImage::new(image_width, image_height);

    let scale = 1.0 / (samples_per_pixel as f64);

    for (i, (_, _, pixel)) in rgb_image.enumerate_pixels_mut().enumerate() {
        let scaled_color = buffer[i] * scale;
        let r = scaled_color.x.sqrt();
        let g = scaled_color.y.sqrt();
        let b = scaled_color.z.sqrt();

        *pixel = Rgb([clamp_color(r), clamp_color(g), clamp_color(b)]);
    }

    rgb_image.save(filename).expect("File write error");
}

pub fn random_double() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

pub fn random_double_from_range(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
