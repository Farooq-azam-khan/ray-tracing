use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList {
    pub list: Vec<HitRecord>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { list: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }

    pub fn add(&mut self, x: HitRecord) {
        self.list.push(x);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closes_so_far = t_max;
        for obj in &self.list {
            if let Some(rec) = obj.hit(&r, t_min, t_max) {
                closes_so_far = rec.t;
                hit_record = Some(rec);
            }
        }
        hit_record
    }
}
