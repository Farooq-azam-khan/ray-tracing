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

 use crate::hittable::{HitRecord,  Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::material::Material; 

#[derive(Copy, Debug, Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Material 
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> Sphere {
        Sphere { center, radius , material }
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
                    material: self.material 
                });
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp, 
                    p: r.at(temp), 
                    normal: (r.at(temp) - self.center) / self.radius, 
                    material: self.material 
                })
            } 
        }
        None 
    }
}
