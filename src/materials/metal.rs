use crate::materials::material::Scatterable;
use crate::structures::{hittable::HitRecord, ray::Ray, vec3::Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Metal {
        return Metal { albedo };
    }
}

impl Scatterable for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(&r_in.direction.unit_vector(), &rec.normal);
        *scattered = Ray::new(rec.point, reflected);
        *attenuation = self.albedo;
        return scattered.direction.dot(&rec.normal) > 0.0;
    }
}
