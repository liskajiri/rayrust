pub mod ray;
pub mod vec3;

use ray::*;
use vec3::*;

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    // Calculating if a sphere can ever be hit by a ray

    let shifted_center = ray.origin() - center;
    let a = ray.direction().length_squared();
    let half_b = dot(shifted_center, ray.direction());
    let c = dot(shifted_center, shifted_center) - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(r: Ray) -> Color {
    let sphere_center = Point3::z(-1.0);
    let t = hit_sphere(sphere_center, 0.5, &r);
    if t > 0.0 {
        let n = (r.at(t) - Vec3::z(-1.0)).unit_vector();
        return 0.5
            * Color {
                x: n.x + 1.0,
                y: n.y + 1.0,
                z: n.z + 1.0,
            };
    }

    let unit_direction = r.direction().unit_vector();
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
