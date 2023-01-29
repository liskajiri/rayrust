use ray::*;
use vec3::*;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::sphere::Sphere;
use crate::utilities::{random_double, random_double_from_range, write_buffer_to_file};

mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod utilities;
mod vec3;

fn random_scene() -> HittableList {
    // World
    let mut world = HittableList::EMPTY;

    let material_ground = Box::new(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });
    world.add(Box::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: -1000.0,
            z: -1.0,
        },
        radius: 1000.0,
        material: material_ground,
    }));

    let center_comparison_pt = Point3 {
        x: 4.0,
        y: 0.2,
        z: 0.0,
    };

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random_double();

            let center = Point3 {
                x: (a as f64) + 0.9 * random_double(),
                y: 0.2,
                z: (b as f64) + 0.9 * random_double(),
            };

            if (center - center_comparison_pt).length() > 0.9 {
                let sphere_material: Box<dyn Material>;

                if choose_material < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Box::new(Lambertian { albedo });
                } else if choose_material < 0.95 {
                    // metal
                    let albedo = Color::random_from_range(0.5, 1.0);
                    let fuzziness = random_double_from_range(0.0, 0.5);
                    sphere_material = Box::new(Metal { albedo, fuzziness });
                } else {
                    sphere_material = Box::new(Dielectric {
                        index_of_refraction: 1.5,
                    });
                }
                world.add(Sphere::boxed(center, 0.2, sphere_material));
            }
        }
    }

    let material_1 = Box::new(Dielectric {
        index_of_refraction: 1.5,
    });
    world.add(Sphere::boxed(Point3::y(1.0), 1.0, material_1));

    let material_2 = Box::new(Lambertian {
        albedo: Color::new(0.4, 0.4, 0.1),
    });
    world.add(Sphere::boxed(
        Point3 {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        1.0,
        material_2,
    ));

    let material_3 = Box::new(Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzziness: 0.0,
    });
    world.add(Sphere::boxed(
        Point3 {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        1.0,
        material_3,
    ));

    world
}

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
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let world = random_scene();
    // Camera
    let look_from = Vec3 {
        x: 13.0,
        y: 2.0,
        z: 3.0,
    };
    let look_at = Vec3::ZERO;
    let dist_to_focus = 10.0;
    let view_up = Vec3::y(1.0);
    let aperture = 0.1;
    let camera = Camera::new(
        look_from,
        look_at,
        view_up,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

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
    let image_name = "image_21";
    write_buffer_to_file(
        &format!("images/{}.ppm", image_name),
        &buffer,
        samples_per_pixel,
        image_width,
        image_height,
    );

    // let png_path = &format!("tests/{}.png", image_name);
    // save_as_png(png_path, &buffer, samples_per_pixel);
}
