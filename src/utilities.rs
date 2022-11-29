use rand::Rng;

use std::fs::OpenOptions;
use std::io::Write;

use crate::vec3::Color;

pub fn write_buffer_to_file(filepath: &String, buffer: &Vec<Color>, samples_per_pixel: u32) {
    fn clamp_color(color: f64) -> i32 {
        (256.0 * clamp(color, 0.0, 0.999)) as i32
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(filepath)
        .unwrap();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let first_line = format!("P3\n{image_width} {image_height}\n255\n");
    file.write_all(first_line.as_bytes()).expect("No file");

    let scale = 1.0 / (samples_per_pixel as f64);

    let mut output = String::new();

    for pixel_color in buffer {
        let scaled_color = *pixel_color * scale;
        let formatted_color = format!(
            "{} {} {}\n",
            clamp_color(scaled_color.x),
            clamp_color(scaled_color.y),
            clamp_color(scaled_color.z),
        );
        output.push_str(&*formatted_color);
    }
    file.write_all(output.as_bytes()).unwrap();
}

pub fn random_double() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
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
