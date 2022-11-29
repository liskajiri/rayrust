use std::fs::OpenOptions;
use std::io::Write;

use crate::vec3::Color;

pub fn write_buffer_to_file(filepath: &String, buffer: &Vec<Color>) {
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

    let const_260 = 255.999;
    for pixel_color in buffer {
        let formatted_color = format!(
            "{} {} {}",
            ((const_260 * pixel_color.x) as i32),
            ((const_260 * pixel_color.y) as i32),
            ((const_260 * pixel_color.z) as i32)
        );

        file.write_all(formatted_color.as_bytes()).unwrap();
        file.write_all("\n".as_bytes()).unwrap();
    }
}
