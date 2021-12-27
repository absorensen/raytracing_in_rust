use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use minifb::clamp;
use rand::Rng;

pub type Point3 = Vector3;
pub type Color = Vector3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn zero() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn one() -> Vector3 {
        Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn length_squared(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt();
    }

    pub fn dot(u: &Vector3, v: &Vector3) -> f64 {
        return u.x * v.x + u.y * v.y + u.z * v.z;
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Vector3{x: rng.gen::<f64>(), y: rng.gen::<f64>(), z: rng.gen::<f64>() }
    }

    pub fn random_min_max(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        let x_rand = rng.gen::<f64>();
        let x = max * x_rand + min * (1.0 - x_rand);

        let y_rand = rng.gen::<f64>();
        let y = max * y_rand + min * (1.0 - y_rand);

        let z_rand = rng.gen::<f64>();
        let z = max * z_rand + min * (1.0 - z_rand);

        Vector3{x: x, y: y, z: z }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let candidate = Vector3::random_min_max(-1.0, 1.0);
            if candidate.length_squared() < 1.0 { return candidate; }
        }
    }

    pub fn random_unit_vector() -> Self {
        Vector3::random_in_unit_sphere().normalized()
    }

    pub fn random_in_hemisphere(normal: &Vector3) -> Self {
        let in_unit_sphere = Vector3::random_in_unit_sphere();
        if Vector3::dot(&in_unit_sphere, normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn cross(u: Vector3, v: Vector3) -> Vector3 {
        return Vector3::new(
            u.y * v.z - u.z * v.y,
            u.z * v.x - u.x * v.z,
            u.x * v.y - u.y * v.x,
        );
    }

    pub fn normalized(&self) -> Vector3 {
        return *self / self.length();
    }

    #[inline]
    pub fn color_to_output(self, image_buffer: &mut Vec<f64>, offset: usize, scale: f64) -> () {
        let r = (scale * self.x).sqrt();
        let g = (scale * self.y).sqrt();
        let b = (scale * self.z).sqrt();

        image_buffer[offset + 0] = (255.999 * clamp(0.0, r, 0.999)) as f64;
        image_buffer[offset + 1] = (255.999 * clamp(0.0, g, 0.999)) as f64;
        image_buffer[offset + 2] = (255.999 * clamp(0.0, b, 0.999)) as f64;
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, factor: f64) -> Vector3 {
        Vector3 {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, factor: f64) {
        *self = Self {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        };
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, factor: f64) -> Vector3 {
        Vector3 {
            x: self.x / factor,
            y: self.y / factor,
            z: self.z / factor,
        }
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, factor: f64) {
        *self = Vector3 {
            x: self.x / factor,
            y: self.y / factor,
            z: self.z / factor,
        };
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}