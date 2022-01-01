use crate::vec3; //::Colour;

pub fn write_colour(colour: vec3::Colour) {
    let ir: u32 = (255.999 * colour.r()) as u32;
    let ig: u32 = (255.999 * colour.g()) as u32;
    let ib: u32 = (255.999 * colour.b()) as u32;
    println!("{} {} {}", ir, ig, ib);
}
