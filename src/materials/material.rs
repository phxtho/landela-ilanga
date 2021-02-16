use crate::materials::lambertian::Lambertian;
use crate::structures::{hittable::HitRecord, ray::Ray, vec3::Vec3};

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian(Lambertian),
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
        match *self {
            Material::Lambertian(ref inner) => inner.scatter(r_in, rec, attenuation, scattered),
        }
    }
}
