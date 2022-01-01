use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};
pub enum HittablType {
    Sphere(Sphere),
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    hittable_type: HittablType,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
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
    pub fn new(p: Point3, normal: Vec3, t: f64, hittable_type: HittablType) -> HitRecord {
        HitRecord {
            p,
            normal,
            t,
            hittable_type,
        }
    }
}

impl Hittable for HitRecord {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self.hittable_type {
            HittablType::Sphere(sp) => sp.hit(r, t_min, t_max),
            _ => None,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = Vec3::dot(r.direction, r.direction);
        let b = Vec3::dot(oc, r.direction);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    p: r.at(temp),
                    normal: (r.at(temp) - self.center) / self.radius,
                    hittable_type: HittablType::Sphere(*self),
                });
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    p: r.at(temp),
                    normal: (r.at(temp) - self.center) / self.radius,
                    hittable_type: HittablType::Sphere(*self),
                });
            }
        }
        None
    }
}
