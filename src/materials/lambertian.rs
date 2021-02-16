use crate::materials::material::Scatterable;
use crate::structures::{hittable::HitRecord, ray::Ray, vec3::Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_in_unit_sphere().unit_vector();
        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.point, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}
