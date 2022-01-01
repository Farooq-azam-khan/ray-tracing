mod colour;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod utils;
mod vec3;

use colour::write_colour;
use hittable::{HitRecord, HittablType, Hittable};
use hittable_list::HittableList;
use ray::Ray;
// use sphere::hit_sphere;
use vec3::{Colour, Point3, Vec3};

fn ray_colour(r: Ray, world: &HittableList) -> Colour {
    if let Some(hit_record) = world.hit(&r, 0.0, f64::INFINITY) {
        return 0.5 * (hit_record.normal + Colour::new(1.0, 1.0, 1.0));
    }
    let unit_dir = Vec3::unit_vector(r.direction);
    let t = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
}

fn draw_image() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

    // World
    let world = make_world();
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
            let pixel_colour = ray_colour(r, &world);
            write_colour(pixel_colour);
        }
    }
    eprintln!("Done!");
}

fn main() {
    draw_image();
}

fn make_world() -> HittableList {
    let mut world: HittableList = HittableList::new();
    let p = Point3::new(0.0, 0.0, -1.0);
    let hr = HitRecord::new(
        p,
        Vec3::new(0.0, 0.0, 0.0),
        0.0,
        HittablType::Sphere(sphere::Sphere::new(p, 0.5)),
    );
    let p2 = Point3::new(0.0, -100.5, -1.0);
    let hr2 = HitRecord::new(
        p2,
        Vec3::new(0.0, 0.0, 0.0),
        0.0,
        HittablType::Sphere(sphere::Sphere::new(p2, 100.0)),
    );
    world.add(hr2);
    world.add(hr);

    world
}
