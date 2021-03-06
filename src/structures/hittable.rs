use crate::materials::{lambertian::Lambertian, material::Material};
use crate::structures::ray::Ray;
use crate::structures::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        return HitRecord {
            point: Vec3::default(),
            normal: Vec3::default(),
            material: Material::Lambertian(Lambertian::new(Vec3::random_range(0.0, 1.0) * 255.0)),
            t: f64::default(),
            front_face: bool::default(),
        };
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_nomral: &Vec3) {
        self.front_face = r.direction.dot(outward_nomral) < 0.0;
        self.normal = if self.front_face {
            *outward_nomral
        } else {
            -*outward_nomral
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(r, t_min, closest_so_far, rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }

        return hit_anything;
    }
}
