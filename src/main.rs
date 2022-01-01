mod colour;
mod ray;
mod sphere;
mod vec3;

use colour::write_colour;
use ray::Ray;
use sphere::hit_sphere;
use vec3::{Colour, Point3, Vec3};

fn ray_colour(r: Ray) -> Colour {
    let sphere_center: Point3 = Point3::new(0.0, 0.0, -1.0);
    let radius = 0.5;
    // check if ray hit the sphere
    if hit_sphere(&sphere_center, radius, &r) {
        return Colour::new(1.0, 0.0, 0.0);
    }

    let unit_dir = Colour::unit_vector(r.direction);
    let t: f64 = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
}

fn draw_image() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        eprintln!("Scanlines Remaining: {}", j);
        for i in 0..image_width {
            let u = (i as f64) / (image_width as f64 - 1.0);
            let v = (j as f64) / (image_height as f64 - 1.0);
            let r: Ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_colour = ray_colour(r);
            write_colour(pixel_colour);
        }
    }
    eprintln!("Done!");
}

fn main() {
    draw_image();
}
