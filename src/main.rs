use ray::*;
use vec3::*;

use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::utilities::write_buffer_to_file;

pub mod hittable;
pub mod hittable_list;
pub mod ray;
pub mod sphere;
pub mod utilities;
pub mod vec3;

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

    // World
    let mut world = HittableList::EMPTY;
    world.add(Box::new(Sphere::new(Point3::z(-1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

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
    let mut buffer: Vec<Color> = Vec::new();

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
            let pixel_color = ray_color(&r, &world);

            buffer.push(pixel_color);
        }
    }

    write_buffer_to_file(&"./images/Image_5.ppm".to_string(), &buffer);
}
