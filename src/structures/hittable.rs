use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;

pub struct HitRecord {
    pub point : Vec3,
    pub normal: Vec3,
    pub t: f64
}


pub trait Hittable {
    fn hit (&self, r :&Ray, t_min : f64, t_max :f64, rec : &mut HitRecord ) -> bool;
}