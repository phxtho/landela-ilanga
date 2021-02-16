use crate::structures::{hittable::HitRecord, ray::Ray, vec3::Vec3};

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian,
}

pub trait Scatterable {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}
impl Scatterable for Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        return false;
    }
}
