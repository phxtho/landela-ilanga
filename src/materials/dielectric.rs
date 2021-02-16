use crate::materials::material::Scatterable;
use crate::structures::{hittable::HitRecord, ray::Ray, vec3::Vec3};
use crate::utils::random_double;

#[derive(Clone, Copy, Debug)]
pub struct Dielectric {
    pub ir: f64, // index of refraction
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            ir: index_of_refraction,
        }
    }

    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
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
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            self.ir
        } else {
            1.0 / self.ir
        };

        let unit_direction = r_in.direction.unit_vector();
        let rn = -unit_direction.dot(&rec.normal);
        let cos_theta = if rn < 1.0 { rn } else { 1.0 };
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random_double()
        {
            direction = Vec3::reflect(&unit_direction, &rec.normal);
        } else {
            direction = Vec3::refract(&unit_direction, &rec.normal, refraction_ratio);
        }

        *scattered = Ray::new(rec.point, direction);
        return true;
    }
}
