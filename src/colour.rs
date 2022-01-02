use crate::utils::clamp;
use crate::vec3::Colour;

pub fn write_colour(colour: Colour, samples_per_pixel: u32) {
    let mut r = colour.r();
    let mut g = colour.g();
    let mut b = colour.b();

    // divide colour by number of samples
    let scale = 1.0 / samples_per_pixel as f64;
    r *= (scale * r).sqrt();
    g *= (scale * g).sqrt();
    b *= (scale * b).sqrt();

    let ir: u32 = (256.0 * clamp(r, 0.0, 0.999)) as u32;
    let ig: u32 = (256.0 * clamp(g, 0.0, 0.999)) as u32;
    let ib: u32 = (256.0 * clamp(b, 0.0, 0.999)) as u32;

    println!("{} {} {}", ir, ig, ib);
}
