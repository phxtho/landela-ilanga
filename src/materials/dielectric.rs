use crate::materials::material::Scatterable;
use crate::structures::{hittable::HitRecord, ray::Ray, vec3::Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            ir: index_of_refraction,
        }
    }
}

impl Scatterable for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        attenuation = Vec3::new(1.0,1.0,1.0);
        let refraction_ratio = if rec.front_face {self.ir} else {1.0/self.ir};

        let unit_direction = r_in.direction().unit_vector();
        let refracted = Vec3::refract(unit_direction, rec.normal, refraction_ratio);
    }
}
