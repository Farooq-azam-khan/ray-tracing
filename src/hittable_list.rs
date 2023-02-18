use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { list: Vec::new() }
    }

    pub fn _clear(&mut self) {
        self.list.clear();
    }

    pub fn add(&mut self, x: Box<dyn Hittable>) {
        self.list.push(x);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false; 
        let mut closes_so_far = t_max;
        for obj in &self.list {
            if obj.hit(&r, t_min, t_max, &mut temp_rec) {
                hit_anything = true; 
                closes_so_far = temp_rec.t;
                rec.t = temp_rec.t; 
                rec.p = temp_rec.p; 
                rec.normal = temp_rec.normal; 
            }
        }
       hit_anything  
    }
}
