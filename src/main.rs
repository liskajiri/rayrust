use ray::*;
use vec3::*;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::utilities::{random_double, write_buffer_to_file};

mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod utilities;
mod vec3;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        return 0.5 * (rec.normal + Color::ONE);
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::ONE + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    // World
    let mut world = HittableList::EMPTY;
    world.add(Box::new(Sphere::new(Point3::z(-1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new();

    // Render
    let mut buffer: Vec<Color> = Vec::new();

    let image_width_less = (image_width as f64) - 1.0;
    let image_height_less = (image_height as f64) - 1.0;

    for j in (0..image_height).rev() {
        eprintln!("Scan lines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = Color::ZERO;
            for _ in 0..samples_per_pixel {
                let u = ((i as f64) + random_double()) / image_width_less;
                let v = ((j as f64) + random_double()) / image_height_less;

                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }
            buffer.push(pixel_color);
        }
    }

    write_buffer_to_file(
        &"./images/image_6.ppm".to_string(),
        &buffer,
        samples_per_pixel,
    );
}
