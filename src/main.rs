#![warn(rust_2018_idioms)]
#![allow(elided_lifetimes_in_paths)]

use rayon::prelude::*;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Material, Materials, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::utilities::{
    _save_as_png, random_double, random_double_from_range, write_buffer_to_file,
};
use crate::vec3::{Color, Point3, Vec3};

mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod utilities;
mod vec3;

fn final_scene() -> HittableList {
    // World
    let mut world = HittableList::EMPTY;

    let material_ground = Materials::Lambertian(Lambertian::new(&Color::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, -1.0),
        1000.0,
        material_ground,
    ));

    let center_comparison_pt = Point3::new(4.0, 0.2, 0.0);

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random_double();

            let center = Point3::new(
                (a as f64) + 0.9 * random_double(),
                0.2,
                (b as f64) + 0.9 * random_double(),
            );

            if (center - center_comparison_pt).length() > 0.9 {
                let sphere_material: Materials;

                if choose_material < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Materials::Lambertian(Lambertian { albedo });
                } else if choose_material < 0.95 {
                    // metal
                    let albedo = Color::random_from_range(0.5, 1.0);
                    let fuzziness = random_double_from_range(0.0, 0.5);
                    sphere_material = Materials::Metal(Metal::new(albedo, fuzziness));
                } else {
                    sphere_material = Materials::Dielectric(Dielectric::new(1.5));
                }
                world.add(Sphere::new(center, 0.2, sphere_material));
            }
        }
    }

    let material_1 = Materials::Dielectric(Dielectric::new(1.5));
    world.add(Sphere::new(Point3::y(1.0), 1.0, material_1));

    let material_2 = Materials::Lambertian(Lambertian::new(&Color::new(0.4, 0.4, 0.1)));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material_2));

    let material_3 = Materials::Metal(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material_3));

    world
}

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
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
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 10;
    const MAX_DEPTH: i32 = 5;

    let world = final_scene();
    // Camera
    let camera = Camera::new(
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::ZERO,
        Vec3::y(1.0),
        20.0,
        ASPECT_RATIO,
        0.1,
        10.0,
    );

    let mut buffer = Vec::with_capacity((IMAGE_WIDTH * IMAGE_HEIGHT) as usize);

    for j in (0..IMAGE_HEIGHT).rev() {
        if j % 50 == 0 {
            println!("Line {j}");
        }
        let line_pixels = (0..IMAGE_WIDTH)
            .into_par_iter()
            .map(|i| {
                let pixel: Color = (0..SAMPLES_PER_PIXEL)
                    .map(|_| {
                        let u = ((i as f64) + random_double()) / ((IMAGE_WIDTH - 1) as f64);
                        let v = ((j as f64) + random_double()) / ((IMAGE_HEIGHT - 1) as f64);

                        let r = &camera.get_ray(u, v);
                        let pixel_color = ray_color(r, &world, MAX_DEPTH);
                        pixel_color
                    })
                    .sum::<Color>();
                pixel
            })
            .collect::<Vec<Color>>();

        buffer.extend(line_pixels);
    }

    let image_name = "image_xy";
    write_buffer_to_file(
        &format!("images/{image_name}.ppm"),
        &buffer,
        SAMPLES_PER_PIXEL,
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
    );

    let png_path = &format!("images/{image_name}.png");
    _save_as_png(
        png_path,
        &buffer,
        SAMPLES_PER_PIXEL,
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
    );
}
