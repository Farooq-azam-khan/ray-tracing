use crate::Vec3; 

// P(t) = A + tb
// P == 3D position along a line in 3D
// A == is the ray origin 
// b == ray direction 
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Vec3, 
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }
    pub fn at(self, t: f64) -> Vec3 {
        self.origin + t * self.direction 
    }
}
