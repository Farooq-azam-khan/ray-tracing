mod camera;
mod colour;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod utils;
mod vec3;

use colour::write_colour;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
// use sphere::hit_sphere;
use camera::Camera;
use utils::random_f64;
use vec3::{random_in_unit_sphere, Colour, Point3, Vec3};

fn ray_colour(r: Ray, world: &HittableList, depth: u32) -> Colour {
    if depth <= 0 {
        // eprintln!("Reached Max Depth");
        return Colour::default(); 
    }
    let mut rec = HitRecord::default(); 
    if world.hit(&r, 0.0, f64::MAX, &mut rec) {
        let target: Point3 = rec.p + rec.normal + random_in_unit_sphere();
        return 0.5
            * ray_colour(
                Ray::new(rec.p, target - rec.p),
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
    eprintln!("({}, {})", image_height, image_width); 
    // World
    let world = make_world();

    // Camera
    let cam: Camera = Camera::new(aspect_ratio, 2.0, 1.0);

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        eprintln!("Scanlines Remaining: {}", j);

        for i in 0..image_width {
            if  j <= 177 { 
                eprintln!("Image Width: {}", i); 
            }
            let mut pixel_colour: Colour = Colour::default(); // new(0.0, 0.0, 0.0);
            for s in 0..samples_per_pixel {
                let u = (i as f64 + random_f64()) / image_width as f64 ;
                let v = (j as f64 + random_f64()) / image_height as f64;
                let r = cam.get_ray(u, v);
                if (j <= 177) && (i >= 192) {
                    eprintln!("at sample {}", s); 
                }
                pixel_colour = pixel_colour + ray_colour(r, &world, max_depth);
                if (j <= 177) && (i >= 192) && (s >= 25) {
                    eprintln!("pixel_colour: {:?}", &pixel_colour); 
                }

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
    let p2 = Point3::new(0.0, -100.5, -1.0);
    world.add(Box::new(sphere::Sphere::new(p, 0.5)));
    world.add(Box::new(sphere::Sphere::new(p2, 100.0)));

    world
}
