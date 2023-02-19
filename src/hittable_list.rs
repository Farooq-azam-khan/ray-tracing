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
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64 ) -> Option<HitRecord> {
        let mut hit_anything = false; 
        let mut closest_so_far = t_max;
        let mut hit_record = None; 
        for obj in &self.list {
            if let Some(rec) = obj.hit(r, t_min, closest_so_far ) {
                hit_anything = true; 
                closest_so_far = rec.t;
                hit_record = Some(rec); 
            }
        }
      hit_record 
    }
}
