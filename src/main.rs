use ray::*;
use vec3::*;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::material::{Lambertian, Material, Metal};
use crate::sphere::Sphere;
use crate::utilities::{random_double, write_buffer_to_file};

mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod utilities;
mod vec3;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::ZERO;
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        // scattered and attenuation are changed in material.scatter()
        return if let Some((scattered, attenuation)) = rec.material.scatter(r, &rec) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::ZERO
        };
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
    let max_depth = 50;

    // World
    let mut world = HittableList::EMPTY;

    let material_ground = Box::new(Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    });
    let material_center = Box::new(Lambertian {
        albedo: Color::new(0.7, 0.3, 0.3),
    });
    let material_left = Box::new(Metal {
        albedo: Color::new(0.8, 0.8, 0.8),
        fuzziness: 0.3,
    });
    let material_right = Box::new(Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzziness: 1.0,
    });

    world.add(Box::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
        material: material_ground,
    }));

    world.add(Box::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        material: material_center,
    }));

    world.add(Box::new(Sphere {
        center: Point3 {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        material: material_left,
    }));

    world.add(Box::new(Sphere {
        center: Point3 {
            x: 1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        material: material_right,
    }));

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
                pixel_color += ray_color(&r, &world, max_depth);
            }
            buffer.push(pixel_color);
        }
    }

    write_buffer_to_file(
        &"./images/image_12.ppm".to_string(),
        &buffer,
        samples_per_pixel,
    );
}
