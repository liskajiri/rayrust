mod vec3;

use vec3::*;

fn main() {
    let IMAGE_WIDTH = 256;
    let IMAGE_HEIGHT = 256;

    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    let const_mult = 255.999;

    for j in (0..=IMAGE_HEIGHT - 1).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let pixel_color = Vec3::new(
                (i as f64) / ((IMAGE_WIDTH - 1) as f64),
                (j as f64) / ((IMAGE_HEIGHT - 1) as f64),
                0.25,
            );

            write_color(pixel_color);
        }
    }
}
