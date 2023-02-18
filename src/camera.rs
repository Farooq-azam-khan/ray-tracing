use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    origin: Point3, 
    lower_left_corner: Point3, 
    horizontal: Vec3, 
    vertical: Vec3, 
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0), 
            horizontal: Vec3::new(4.0, 0.0, 0.0), 
            vertical: Vec3::new(0.0, 2.0, 0.0), 
            origin: Vec3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, 
                 self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin
                 )
    }
}

/* 
pub struct Camera {
    aspect_ratio: f64,
    viewport_height: f64,
    focal_length: f64,
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, viewport_height: f64, focal_length: f64) -> Camera {
        let viewport_width = aspect_ratio * viewport_height;
        Camera {
            aspect_ratio,
            viewport_height,
            focal_length,
            origin: Point3::new(0.0, 0.0, 0.0),
            horizontal: Vec3::new(viewport_width, 0.0, 0.0),
            vertical: Vec3::new(0.0, viewport_height, 0.0),
        }
    }
    pub fn get_viewport_height(&self) -> f64 {
        self.viewport_height
    }
    pub fn get_viewport_width(&self) -> f64 {
        self.aspect_ratio * self.viewport_height
    }
    pub fn get_lower_left_corner(&self) -> Vec3 {
        self.origin
            - self.horizontal / 2.0
            - self.vertical / 2.0
            - Vec3::new(0.0, 0.0, self.focal_length)
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.get_lower_left_corner() + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
*/
