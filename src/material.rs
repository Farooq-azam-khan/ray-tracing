use crate::ray; 
use crate::hittable; 
use hittable::HitRecord;
use crate::vec3::{Vec3, Colour, random_unit_vector, random_in_unit_sphere}; 
use ray::Ray; 

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Lambertian {albedo: Colour},
    Metal {albedo: Colour, fuzz: f64 }
}

pub fn scatter(material: &Material, 
            ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
    match material {
        Material::Lambertian {albedo} => {
            let rand_v = random_unit_vector(); 
            if (rec.normal + rand_v).near_zero() {
                let scatter_direction = rec.normal ;
                *scattered = Ray::new(rec.p, scatter_direction); 
                *attenuation = albedo.clone(); 
                return true; 
            } else {
                let scatter_direction = rec.normal + rand_v; 
                *scattered = Ray::new(rec.p, scatter_direction); 
                *attenuation = albedo.clone(); 
                return true; 
            }
        },
        Material::Metal {albedo, fuzz } => {
            let reflected = Vec3::reflect(Vec3::unit_vector(ray_in.direction), rec.normal); 
            *scattered = Ray::new(rec.p, reflected + *fuzz*random_in_unit_sphere()); 
            *attenuation = albedo.clone(); 
            Vec3::dot(scattered.direction, rec.normal) > 0.0
        }
    }
    
}


