mod vec3; 
use vec3::{Vec3}; 

fn draw_image() {
    // Image 
    let image_width = 256; 
    let image_height = 256; 

    // Renger 
    println!("P3\n{} {}\n255", image_width, image_height); 

    for j in (0..image_width).rev() {
        eprintln!("Scanlines remaining: {}", j); 
        for i in 0..image_height {
            let r = (i as f64) / ((image_width - 1) as f64); 
            let g = (j as f64) / ((image_height -1) as f64); 
            let b = 0.25; 

            let ir = (255.999 * r) as i64; 
            let ig = (255.999 * g) as i64; 
            let ib = (255.999 * b) as i64; 

            println!("{} {} {}", ir, ig, ib); 

        }
    }
    eprintln!("Done"); 


}

fn main() {
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

}
