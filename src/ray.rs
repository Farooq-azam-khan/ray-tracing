use crate::vec3;
use vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}

#[test]
fn test_ray_creation() {
    Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
}

#[test]
fn test_ray_at_orig_point() {
    let p: Point3 = Point3::new(0.0, 0.0, 0.0);
    let r = Ray::new(p, Vec3::new(0.0, 1.0, 0.0));
    assert_eq!(r.at(0.0), p);
}

#[test]
fn test_ray_at_t_eq_1() {
    let p: Point3 = Point3::new(0.0, 0.0, 0.0);
    let r: Ray = Ray::new(p, Vec3::new(0.0, 1.0, 0.0));
    assert_eq!(r.at(1.0), Point3::new(0.0, 1.0, 0.0));
}
