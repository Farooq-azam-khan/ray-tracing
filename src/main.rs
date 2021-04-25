fn main() {
    // Image 
    let image_width = 256; 
    let image_height = 256; 

    // Renger 
    println!("P3\n{} {}\n255", image_width, image_height); 

    //let mut j = image_height - 1; 
    //while j >= 0 {
    for j in (0..image_width).rev() {
        //j -= 1; 
        //let mut i = 0; 
        //while i < image_width {
        for i in 0..image_height {
            //i += 1; 
            let r = (i as f64) / ((image_width - 1) as f64); 
            let g = (j as f64) / ((image_height -1) as f64); 
            let b = 0.25; 

            let ir = (255.999 * r) as i64; 
            let ig = (255.999 * g) as i64; 
            let ib = (255.999 * b) as i64; 

            println!("{} {} {}", ir, ig, ib); 

        }
    }
}
