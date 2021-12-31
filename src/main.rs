fn draw_image() {
    // Image
    let image_width: u32 = 256;
    let image_height: u32 = 256;

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        eprintln!("Scanlines Remaining: {}", j);
        for i in 0..image_width {
            let r = (i as f32) / (image_width as f32 - 1.0);
            let g = (j as f32) / (image_height as f32 - 1.0);
            let b = 0.25;

            let ir: u32 = (255.999 * r) as u32;
            let ig: u32 = (255.999 * g) as u32;
            let ib: u32 = (255.999 * b) as u32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("Done!");
}

fn main() {
    draw_image();
}
