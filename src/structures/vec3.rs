use crate::utils::{random_double, random_range};
use std::cmp::PartialEq;
use std::f64;
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    pub elements: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 {
            elements: [e0, e1, e2],
        }
    }

    pub fn x(&self) -> f64 {
        self.elements[0]
    }

    pub fn y(&self) -> f64 {
        self.elements[1]
    }

    pub fn z(&self) -> f64 {
        self.elements[2]
    }

    pub fn r(&self) -> f64 {
        self.elements[0]
    }

    pub fn g(&self) -> f64 {
        self.elements[1]
    }

    pub fn b(&self) -> f64 {
        self.elements[2]
    }

    pub fn print(&self) -> () {
        println!("{:?}", self);
    }
}

// Operators
impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.x() == other.x() && self.y() == other.y() && self.z() == other.z()
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            elements: [
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z(),
            ],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            elements: [
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z(),
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            elements: [
                self.x() - other.x(),
                self.y() - other.y(),
                self.z() - other.z(),
            ],
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            elements: [-self.x(), -self.y(), -self.z()],
        }
    }
}

/// Vector multiplication
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            elements: [
                self.x() * other.x(),
                self.y() * other.y(),
                self.z() * other.z(),
            ],
        }
    }
}

/// Scalar multiplication
impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            elements: [self.x() * other, self.y() * other, self.z() * other],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            elements: [self * other.x(), self * other.y(), self * other.z()],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        if other == 0.0 {
            return Vec3 {
                elements: [f64::MAX, f64::MAX, f64::MAX],
            };
        }
        Vec3 {
            elements: [self.x() / other, self.y() / other, self.z() / other],
        }
    }
}

/// Indexing
impl Index<usize> for Vec3 {
    type Output = f64;
    fn index<'a>(&'a self, i: usize) -> &'a f64 {
        &self.elements[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut f64 {
        &mut self.elements[i]
    }
}

// Utilities
impl Vec3 {
    pub fn length(&self) -> f64 {
        (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn unit_vector(&self) -> Vec3 {
        Vec3 {
            elements: [
                self.x() / self.length(),
                self.y() / self.length(),
                self.z() / self.length(),
            ],
        }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        (self.x() * other.x()) + (self.y() * other.y()) + (self.z() * other.z())
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            elements: [
                self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x(),
            ],
        }
    }

    /// Converts floats to ints
    pub fn colourize(&mut self) -> () {
        self.elements[0] = self.elements[0].floor();
        self.elements[1] = self.elements[1].floor();
        self.elements[2] = self.elements[2].floor();
    }

    pub fn random() -> Vec3 {
        return Vec3::new(random_double(), random_double(), random_double());
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        return Vec3::new(
            random_range(min, max),
            random_range(min, max),
            random_range(min, max),
        );
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s = 1e-8;
        return self.x().abs() < s && self.y().abs() < s && self.z().abs() < s;
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        return *v - (2. * v.dot(n) * *n);
    }

    // implementation of snells law
    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let ab = -uv.unit_vector().dot(n);
        let cos_theta: f64 = if ab < 1.0 { ab } else { 1.0 };
        let r_out_perp = etai_over_etat * (*uv + (cos_theta * *n));
        let discriminant: f64 = (1.0 - r_out_perp.length_squared() as f64).abs();
        let r_out_parrallel: Vec3 = -discriminant.sqrt() * *n;
        return r_out_perp + r_out_parrallel;
    }
}
