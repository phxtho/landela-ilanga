use std::cmp::PartialEq;
use crate::structures::vec3::Vec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray{origin: origin, direction: direction}
    }

    pub fn at(&self, t:f64) -> Vec3 {
        self.origin + t * self.direction
    }
}

impl PartialEq for Ray {
    fn eq(&self, other: &Ray) -> bool {
        self.origin == other.origin && self.direction == other.direction
    }
}
