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
    eprintln!("getting lower left"); 
    let h2 = horizontal*(1.0/2.0); 
    let v2 = vertical*(1.0/2.0); 
    eprintln!("h/2: {:?}, v/2: {:?}", h2, v2); 
    let lower_left_corner = origin - h2 - v2 - Vec3::new(0.0, 0.0, focal_length); 


    // Renger 
    println!("P3\n{} {}\n255", image_width, image_height); 

    for j in (0..image_width).rev() {
        eprintln!("Scanlines remaining: {}", j); 
        for i in 0..image_height {
            let u = (i as f64) / ((image_width - 1) as f64); 
            let v = (j as f64) / ((image_height -1) as f64); 

            let ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin); 
            let ray_c = ray_color(ray); 
            write_color(ray_c); 


            //println!("{} {} {}", ir, ig, ib); 

        }
    }
    eprintln!("Done"); 


}

fn main() {
    /*
    let v1 = Vec3::new(1.0, 2.0, 3.0); 
    let v2 = Vec3::new(1.0, 2.0, 3.0); 
    let v3 = v1 + v2; 
    let v4 = v1 - v2; 
    println!("v3: {:?}", v3); 
    println!("v4: {:?}", v4); 

    let mut vself = Vec3::new(-1.2, 3.0, 1.3); 
    vself += Vec3::new(1.2, -3.0, -1.3); 
    println!("AddAssign: {:?}", vself); 

    let mut vself2 = Vec3::new(-1.0, -1.0, -2.0);
    vself2 *= -1.0; 
    println!("MulAssign: {:?}", vself2); 

    let va = Vec3::new(1.0, 2.0, 3.0); 
    println!("Mul (vec*float): {:?}", va*2.0); 
    println!("Mul (float*vec): {:?}", 2.0*va); 
    println!("lenth: {:?}", Vec3::new(1.0,1.0,1.0).length()); 
    
    println!("Mul (vec*vec): {:?}", Vec3::new(1.0,1.0,1.0) * Vec3::new(1.0, 1.0, 1.0));
    println!("dot (vec*vec): {:?}", Vec3::dot(Vec3::new(1.0,1.0,1.0) , Vec3::new(1.0, 1.0, 1.0))); 
    
    let r = Ray::new(Vec3::new(1.0,1.0,1.0), Vec3::new(1.0, 0.0, 0.0)); 
    println!("{:?}", r.at(0.0)); 
    */

    draw_image();


}

fn write_color(pixel_color: Vec3) {
    let ir = (255.999 * pixel_color.x()) as i64; 
    let ig = (255.999 * pixel_color.y()) as i64; 
    let ib = (255.999 * pixel_color.z()) as i64; 
    println!("{} {} {}", ir, ig, ib); 
}

fn ray_color(r: Ray) -> Vec3 {
    let unit_len = Vec3::unit_vector(r.direction);
    let t = 0.5 * (unit_len.y() + 1.0); 

    let v1 = Vec3::new(1.0, 1.0, 1.0); 
    let v2 = Vec3::new(0.5, 0.7, 1.0); 
    (1.0-t) * v1 + t * v2
}
