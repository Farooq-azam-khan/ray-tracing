use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, _t_min: f64, _t_max: f64, _rec: &mut HitRecord) -> bool {
        false 
    }
}


