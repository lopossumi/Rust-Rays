use crate::vector::*;
use crate::ray::*;
use crate::hittable::{ Record, Hittable };

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { 
            center: center, 
            radius : radius
        }
    }
}

const T_MAX: f64 = 9999999.0;
const T_MIN: f64 = 0.0001;


impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<Record> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        
        if discriminant > 0.0 {

            let root = discriminant.sqrt();
            let mut temp = (-half_b - root)/a;
            if temp < T_MAX && temp > T_MIN {
                let t = temp;
                let p = ray.at(t);
                let normal = (p - self.center) / self.radius;
                
                return Some(Record::new(p, normal, t));
            }
            temp = (-half_b + root) / a;
            if temp < T_MAX && temp > T_MIN {
                let t = temp;
                let p = ray.at(t);
                let normal = (p - self.center) / self.radius;
                
                return Some(Record::new(p, normal, t));
            }
        }

        None  
    }
}