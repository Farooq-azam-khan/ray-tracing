mod camera;
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
use camera::Camera;
use utils::random_f64;
use vec3::{random_in_unit_sphere, Colour, Point3, Vec3};

fn ray_colour(r: Ray, world: &HittableList, depth: u32) -> Colour {
    if depth <= 0 {
        eprintln!("Reached Max Depth");
        return Colour::new(0.0, 0.0, 0.0);
    }
    if let Some(hit_record) = world.hit(&r, 0.0, f64::INFINITY) {
        let target: Point3 = hit_record.p + hit_record.normal + random_in_unit_sphere();
        return 0.5
            * ray_colour(
                Ray::new(hit_record.p, target - hit_record.p),
                &world,
                depth - 1,
            );
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
    let samples_per_pixel: u32 = 100;
    let max_depth = 50;

    // World
    let world = make_world();

    // Camera
    let cam: Camera = Camera::new(aspect_ratio, 2.0, 1.0);

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        eprintln!("Scanlines Remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_colour: Colour = Colour::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u = (i as f64 + random_f64()) / (image_width as f64 - 1.0);
                let v = (j as f64 + random_f64()) / (image_height as f64 - 1.0);
                let r: Ray = cam.get_ray(u, v);
                pixel_colour = pixel_colour + ray_colour(r, &world, max_depth);
            }
            if j == 176 {
                eprintln!("\twriting colour:{}", i);
            }
            write_colour(pixel_colour, samples_per_pixel);
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
