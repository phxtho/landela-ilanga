use crate::structures::{ray::Ray, vec3::Vec3};
use crate::structures::hittable::{HitRecord, Hittable};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64
}

impl Sphere {
    pub fn new (center : Vec3, radius: f64) -> Sphere {
        Sphere{center,radius}
    }
}

impl Hittable for Sphere {
    fn hit (&self,r :&Ray, t_min : f64, t_max :f64, rec :&mut HitRecord ) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a*c;
        if discriminant < 0.0 {
            return false;
        }
        
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) /a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) /a;
            if root < t_min || t_max < root {
                return false
            }
        }

        rec.t = root;
        rec.point = r.at(rec.t);
        rec.normal = (rec.point - self.center)/self.radius;

        return true
    }
}

