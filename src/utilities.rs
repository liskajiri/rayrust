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
        .open(filepath)
        .unwrap();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let mut output = format!("P3\n{image_width} {image_height}\n255\n");

    let scale = 1.0 / (samples_per_pixel as f64);

    for pixel_color in buffer {
        let scaled_color = *pixel_color * scale;
        let r = scaled_color.x.sqrt();
        let g = scaled_color.y.sqrt();
        let b = scaled_color.z.sqrt();
        let formatted_color =
            format!("{} {} {}\n", clamp_color(r), clamp_color(g), clamp_color(b),);
        output.push_str(&*formatted_color);
    }
    file.write_all(output.as_bytes()).unwrap();
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
