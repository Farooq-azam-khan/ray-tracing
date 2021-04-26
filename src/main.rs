mod vec3; 
mod ray; 
use vec3::{Vec3}; 
use ray::{Ray}; 

fn draw_image() {
    // Image 
    let aspect_ratio = 16.0 / 9.0; 

    let image_width = 400; 
    let image_height = (image_width as f64 / aspect_ratio as f64) as i64; 

    // Camera
    let viewport_height = 2.0; 
    let viewport_width = aspect_ratio * viewport_height; 
    let focal_length = 1.0; 

    let origin = Vec3::new(0.0,0.0,0.0); 
    let horizontal = Vec3::new(viewport_width, 0.0,0.0); 
    let vertical = Vec3::new(0.0, viewport_height, 0.0); 
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length); 


    // Renger 
    println!("P3\n{} {}\n255", image_width, image_height); 

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j); 
        for i in 0..image_width {
            let u = (i as f64) / ((image_width - 1) as f64); 
            let v = (j as f64) / ((image_height -1) as f64); 

            let ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin); 
            let ray_c = ray_color(ray); 
            write_color(ray_c); 

        }
    }
    eprintln!("Done"); 


}

fn main() {
  
    draw_image();


}


fn write_color(pixel_color: Vec3) {
    let ir = (255.999 * pixel_color.x()) as i64; 
    let ig = (255.999 * pixel_color.y()) as i64; 
    let ib = (255.999 * pixel_color.z()) as i64; 
    println!("{} {} {}", ir, ig, ib); 
}

/*
    Linearly blends white and blue depending on the height y
    y is scaled between -1 and 1
    scale t such that t=0 is white and t=1 is blue 
*/
fn ray_color(r: Ray) -> Vec3 {
    let center_canvas = Vec3::new(0.0, 0.0, -1.0); // point
    if hit_sphere(center_canvas, 0.5, r) {
        return Vec3::new(1.0, 0.0, 0.0);
    }
    let unit_len = Vec3::unit_vector(r.direction);
    let t = 0.5 * (unit_len.y() + 1.0); 

    let v1 = Vec3::new(1.0, 1.0, 1.0); 
    let v2 = Vec3::new(0.5, 0.7, 1.0); 
    return (1.0-t) * v1 + t * v2;
}


/*
t^2 b dot b + 2t b dot (A-C) + (A-C) dot (A-C) - r^2 = 0
where P(t) = A+tb

*/

fn hit_sphere(center: Vec3, radius: f64, r: Ray) -> bool {
    let oc: Vec3 = r.origin - center; 
    let a: f64 = Vec3::dot(r.direction, r.direction);
    // eprintln!("{:?} * {:?} = {:?}",r.direction, r.direction, a);
    let b: f64 = 2.0 * Vec3::dot(oc, r.direction);
    let c: f64 = Vec3::dot(oc, oc) - radius*radius; 
    let discriminant: f64 = b*b - 4.0 * a * c; 
    discriminant > 0.0
}