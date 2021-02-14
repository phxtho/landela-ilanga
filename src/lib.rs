pub mod structures {
    pub mod hittable;
    pub mod ray;
    pub mod vec3;
}

pub mod objects {
    pub mod camera;
    pub mod sphere;
}

pub mod utils {
    use rand::{thread_rng, Rng};
    pub fn random_double() -> f64 {
        let mut rng = thread_rng();
        return rng.gen_range(0.0..1.0);
    }
}
