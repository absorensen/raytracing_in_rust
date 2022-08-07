use std::f32::consts::PI;
use std::iter::Sum;
use std::{fmt};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign, Index, IndexMut};
use minifb::clamp;
use rand::rngs::mock::StepRng;
use rand::{Rng, rngs::ThreadRng};
use rand_chacha::ChaChaRng;

pub type Point3 = Vector3;
//pub type Color = Vector3;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
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


    #[inline]
    pub fn random(rng: &mut ThreadRng) -> Self {
        Vector3{x: rng.gen::<f32>(), y: rng.gen::<f32>(), z: rng.gen::<f32>() }
    }

    #[inline]
    pub fn random_range(rng: &mut ThreadRng, minimum: f32, maximum: f32) -> Self {
        Vector3{x: rng.gen_range(minimum..maximum), y: rng.gen_range(minimum..maximum), z: rng.gen_range(minimum..maximum) }
    }

    #[inline]
    pub fn random_chacha(rng: &mut ChaChaRng) -> Self {
        Vector3{x: rng.gen::<f32>(), y: rng.gen::<f32>(), z: rng.gen::<f32>() }
    }

    #[inline]
    pub fn random_range_chacha(rng: &mut ChaChaRng, minimum: f32, maximum: f32) -> Self {
        Vector3{x: rng.gen_range(minimum..maximum), y: rng.gen_range(minimum..maximum), z: rng.gen_range(minimum..maximum) }
    }

    #[inline]
    pub fn random_step(rng: &mut StepRng) -> Self {
        Vector3{x: rng.gen::<f32>(), y: rng.gen::<f32>(), z: rng.gen::<f32>() }
    }

    #[inline]
    pub fn random_range_step(rng: &mut StepRng, minimum: f32, maximum: f32) -> Self {
        Vector3{x: rng.gen_range(minimum..maximum), y: rng.gen_range(minimum..maximum), z: rng.gen_range(minimum..maximum) }
    }







    #[inline]
    pub fn length_squared(&self) -> f32 {
        return self.x.mul_add(self.x, self.y.mul_add(self.y, self.z * self.z));
    }

    #[inline]
    pub fn length(&self) -> f32 {
        return self.length_squared().sqrt();
    }

    #[inline]
    pub fn dot(u: &Vector3, v: &Vector3) -> f32 {
        return u.x.mul_add(v.x, u.y.mul_add(v.y, u.z * v.z));
    }

    #[inline]
    pub fn cross(u: &Vector3, v: &Vector3) -> Vector3 {
        return Vector3::new(
            u.y.mul_add(v.z, - (u.z * v.y)),
            u.z.mul_add(v.x, - (u.x * v.z)),
            u.x.mul_add(v.y, - (u.y * v.x)),
        );
    }

    #[inline]
    pub fn normalized(&self) -> Vector3 {
        return *self / self.length();
    }


    #[inline]
    pub fn reflect(v: &Vector3, normal: &Vector3, reflected_out: &mut Vector3) -> bool {
        *reflected_out = (*v) - (2.0 * Vector3::dot(v, normal) * (*normal));

        true
    }

    #[inline]
    pub fn refract(v: &Vector3, n: &Vector3, etai_over_etat: f32, refracted_out: &mut Vector3) -> bool {
        let negative_uv = -*v;
        let cos_theta = Vector3::dot(&negative_uv,&n).min(1.0);
        let ray_out_perp = etai_over_etat * (*v + cos_theta * (*n));
        let ray_out_parallel = (1.0 - ray_out_perp.length_squared()).abs().sqrt() * (-(*n));    
        *refracted_out = ray_out_perp + ray_out_parallel;
        
        true
    }

    #[inline]
    pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Self {
        let mut candidate: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
        loop {
            candidate.x = rng.gen_range(-1.0..1.0);
            candidate.y = rng.gen_range(-1.0..1.0);
            candidate.z = rng.gen_range(-1.0..1.0);

            if candidate.length_squared() < 1.0 {
                return candidate;
            }
        }
    }

    #[inline]
    pub fn random_in_unit_disk(rng: &mut ThreadRng) -> Self {
        let mut candidate: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
        loop {
            candidate.x = rng.gen_range(-1.0..1.0);
            candidate.y = rng.gen_range(-1.0..1.0);

            if candidate.length_squared() < 1.0 {
                return candidate;
            }
        }
    }

    #[inline]
    pub fn random_unit_vector(rng: &mut ThreadRng) -> Self {
        Vector3::random_in_unit_sphere(rng).normalized()
    }

    #[inline]
    pub fn random_in_hemisphere(rng: &mut ThreadRng, normal: &Vector3) -> Self {
        let in_unit_sphere = Vector3::random_in_unit_sphere(rng);
        if 0.0 < Vector3::dot(&in_unit_sphere, normal) {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    #[inline]
    pub fn random_cosine_direction(rng: &mut ThreadRng) -> Self {
        let r1 = rng.gen::<f32>();
        let r2 = rng.gen::<f32>();
        let z = (1.0 - r2).sqrt();

        let phi = 2.0 * PI * r1;
        let x = phi.cos() * r2.sqrt();
        let y = phi.sin() * r2.sqrt();
    
        Vector3::new( x, y, z )
    }

    #[inline]
    pub fn color_to_output(self, image_buffer: &mut Vec<f32>, offset: usize, scale: f32) -> () {
        let r = (scale * self.x).sqrt();
        let g = (scale * self.y).sqrt();
        let b = (scale * self.z).sqrt();

        image_buffer[offset + 0] = (255.999 * clamp(0.0, r, 0.999)) as f32;
        image_buffer[offset + 1] = (255.999 * clamp(0.0, g, 0.999)) as f32;
        image_buffer[offset + 2] = (255.999 * clamp(0.0, b, 0.999)) as f32;
    }

    #[inline]
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    #[inline]
    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vector3 {
    #[inline]
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

    #[inline]
    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vector3 {

    #[inline]
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

    #[inline]
    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn mul(self, factor: f32) -> Vector3 {
        Vector3 {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;

    #[inline]
    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl MulAssign<f32> for Vector3 {

    #[inline]
    fn mul_assign(&mut self, factor: f32) {
        *self = Self {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        };
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn div(self, factor: f32) -> Vector3 {
        Vector3 {
            x: self.x / factor,
            y: self.y / factor,
            z: self.z / factor,
        }
    }
}

impl DivAssign<f32> for Vector3 {

    #[inline]
    fn div_assign(&mut self, factor: f32) {
        *self = Vector3 {
            x: self.x / factor,
            y: self.y / factor,
            z: self.z / factor,
        };
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    #[inline]
    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sum for Vector3 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self { x: 0.0, y: 0.0, z: 0.0}, |a, b| Self {
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z,
        })
    }
}

impl Index<usize> for Vector3 {
    type Output = f32;

    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => unreachable!(),
        }
    }
}

impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}