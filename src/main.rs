pub mod ray;
pub mod vec3;

use ray::*;
use vec3::*;

fn ray_color(r: Ray) -> Color {
    let mut dir = r.direction();
    let unit_direction = dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);

    // 1 -> blue; 0 -> white
    // blend = (1-t) * start_val + t * end_val
    // Here we have start_val = Color::ONE, end_val = Color(0.5, 0.7, 1)
    (1.0 - t) * Color::ONE
        + t * Color {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        }
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::ZERO;
    let horizontal = Vec3::x(viewport_width);
    let vertical = Vec3::y(viewport_height);
    let depth = Vec3::z(focal_length);
    let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - depth;

    // Render
    print!("P3\n{image_width} {image_height}\n255\n");

    let image_width_less = (image_width as f64) - 1.0;
    let image_height_less = (image_height as f64) - 1.0;
    for j in (0..=image_height - 1).rev() {
        eprintln!("Scan lines remaining: {}", j);
        for i in 0..image_width {
            let u = (i as f64) / image_width_less;
            let v = (j as f64) / image_height_less;

            let ray_direction = lower_left_corner + u * horizontal + v * vertical - origin;
            let r = Ray {
                orig: origin,
                dir: ray_direction,
            };
            let pixel_color = ray_color(r);

            write_color(pixel_color);
        }
    }
}
