use crate::ray::Ray;
use crate::vector::*;

pub struct Record {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}

impl Record {
    pub fn new(p: Point3, normal: Vec3, t: f64) -> Record {
        Record {
            p: p,
            normal: normal,
            t: t,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray) -> Option<Record>;
}
