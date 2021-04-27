use crate::vec3::Vec3; 
use crate::hittable::{Hitable, HitRecord};
use crate::ray::Ray; 
pub struct Sphere {
    pub center: Vec3, 
    pub radius: f64
}

impl Sphere {
    fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere {
            center, radius
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center; 
        let a = r.direction.length_squared();
        let half_b = Vec3::dot(oc, r.direction);
        let c = oc.length_squared() - self.radius * self.radius; 
        let discriminant = half_b*half_b - a * c; 
        if discriminant < 0 {
            return false
        }
        let sqrtd = discriminant.sqrt(); 
        // find the root in the accepted range of [t_min, t_max]
        let root = (-half_b - sqrtd) / a;
        if root <= t_min || root >= t_max {
            return false; 
        }

        rec.t = root; 
        rec.p = r.at(rec.t)

        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        rec.normal = (rec.p-center) / self.radius; 
        true

    }
}