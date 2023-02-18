use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};

pub enum HittableType {
    Sphere(Sphere),
}
#[derive(Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
//    hittable_type: HittableType,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        false 
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        let front_face = Vec3::dot(r.direction, outward_normal) < 0.0;
        self.normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
    pub fn new(p: Point3, normal: Vec3, t: f64, _hittable_type: HittableType) -> HitRecord {
        HitRecord {
            p,
            normal,
            t,
            //hittable_type,
        }
    }
}
 
