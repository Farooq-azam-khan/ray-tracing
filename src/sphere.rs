/* Equation of a sphere: x^2 + y^2 + z^2 = R^2
given a point (a,b,c)
if a^2 + b^2 + c^2 = R^2 then the point (a,b,c) is on the sphere
else if a^2 + b^2 + c^2 < R^2 point (a,b,c) is inside the sphere
else point is outside the sphere (i.e. a^2 + b^2 + c^2 > R^2)

if the sphere's center is at (Cx,Cy,Cz) then
(x-Cx)^2 + (y-Cy)^2 + (z-Cz)^2 = R^2 (eq 3)
let P = Point3::new(a,b,c)
and C = Point3::new(Cx,Cy,Cz)
therefore (eq 3) => (P-C) dot (P - C) = R^2

We want to know if our ray hits our sphere, i.e.
(P(t) - C) dot (P(t) - C) = r^2
(A + tb - C) dot (A + tb - C) - r^2 = 0
t^2b dot b + 2tb dot (A-C) + (A-C) dot (A-C) - r^2 = 0 (eq 5)
A is known
C is known
b is known
t is unknown
(eq. 5) is a quadratic
i.e. 0 roots (ray does not interact with the sphere),
    1 root (ray hits the sphere at 1 point),
    or 2 roots (ray hits the sphere at 2 points)
*/

// use crate::hittable::{HitRecord, HittablType, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

fn hit_sphere_old(center: &Point3, radius: f64, r: &Ray) -> f64 {
    // t^2b dot b + 2tb dot (A-C) + (A-C) dot (A-C) - r^2 = 0 (eq 5)
    let oc: Vec3 = r.origin - *center; // A - C
    let a = Vec3::dot(r.direction, r.direction); // b dot b
    let b = 2.0 * Vec3::dot(oc, r.direction); // 2*b dot (A-C)
    let c = Vec3::dot(oc, oc) - radius * radius; // (A-C) dot (A-C) - r^2
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}

pub fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    // t^2b dot b + 2tb dot (A-C) + (A-C) dot (A-C) - r^2 = 0 (eq 5)
    let oc: Vec3 = r.origin - *center; // A - C
    let a = r.direction.length_squared(); // Vec3::dot(r.direction, r.direction); // b dot b
    let half_b = Vec3::dot(oc, r.direction);
    // let b = 2.0 * Vec3::dot(oc, r.direction); // 2*b dot (A-C)
    let c = oc.length() - radius * radius; // Vec3::dot(oc, oc) - radius * radius; // (A-C) dot (A-C) - r^2
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / (2.0 * a);
    }
}

#[derive(Copy, Debug, Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

// impl Hitable for Sphere {
//     fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
//         let oc: Vec3 = r.origin - self.center;
//         let a = r.direction.length_squared();
//         let half_b = Vec3::dot(oc, r.direction);
//         let c = oc.length() - self.radius * self.radius; // Vec3::dot(oc, oc) - radius * radius; // (A-C) dot (A-C) - r^2
//         let discriminant = half_b * half_b - a * c;
//         if discriminant < 0.0 {
//             return None;
//         } else {
//             let sqrtd = discriminant.sqrt();
//             let mut root = (-half_b - sqrtd) / a;
//             if root < t_min || root > t_max {
//                 root = (-half_b + sqrtd) / a;
//                 if root < t_min || root > t_max {
//                     return Some(HitRecord);
//                 }
//             }
//             rec.t = root;
//             rec.p = r.at(rec.t);

//             let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
//             rec.set_face_normal(&r, outward_normal);
//             rec.normal = (rec.p - self.center) / self.radius;
//             return true;
//         }
//     }
// }
