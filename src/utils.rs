use rand::Rng;

// Constants
fn pi() -> f64 {
    3.1415926535897932385
}

// Utility functions
fn _degrees_to_radians(deg: f64) -> f64 {
    deg * pi() / 180.0
}

pub fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_f64_range(min: f64, max: f64) -> f64 {
    min + random_f64() * (min - max)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 { if x < min { return min; }
    if x > max {
        return max;
    }
    x
}
