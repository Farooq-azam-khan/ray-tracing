use crate::vec3::Vec3;
use crate::ray::Ray; 

struct HitRecord {
    p: Vec3, // point
    n: Vec3, //normal
    t: f64
}
impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        let front_face = Vec3::dot(r.direction, outward_normal) < 0; 
        self.normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        }
        
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}