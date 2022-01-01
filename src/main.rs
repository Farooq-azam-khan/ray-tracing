mod colour;
mod vec3;

use colour::write_colour;
use vec3::Colour;

fn draw_image() {
    // Image
    let image_width: u32 = 256;
    let image_height: u32 = 256;

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        eprintln!("Scanlines Remaining: {}", j);
        for i in 0..image_width {
            let r = (i as f64) / (image_width as f64 - 1.0);
            let g = (j as f64) / (image_height as f64 - 1.0);
            let b = 0.25;
            let pixel_colour: Colour = Colour::new(r, g, b);
            write_colour(pixel_colour);
        }
    }
    eprintln!("Done!");
}

fn main() {
    draw_image();
}
