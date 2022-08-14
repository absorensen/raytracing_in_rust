use std::f32::consts::PI;
use std::iter::Sum;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign, Index, IndexMut};
use rand::{Rng, rngs::ThreadRng};
use rand_chacha::ChaChaRng;

pub type Point3 = Vector3;

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
        Vector3::new(0.0, 0.0, 0.0)
    }

    #[inline]
    pub fn random_range_chacha(rng: &mut ChaChaRng, minimum: f32, maximum: f32) -> Self {
        Vector3::new(rng.gen_range(minimum..maximum), rng.gen_range(minimum..maximum), rng.gen_range(minimum..maximum) )
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
    pub fn cross_into(u: &Vector3, v: &Vector3, result_out: &mut Vector3) {
        result_out.x = u.y.mul_add(v.z, - (u.z * v.y));
        result_out.y = u.z.mul_add(v.x, - (u.x * v.z));
        result_out.z = u.x.mul_add(v.y, - (u.y * v.x));
    }

    // Maybe optimize with the inverse multiplication or create alternate function
    #[inline]
    pub fn get_normalized(&self) -> Vector3 {
        return *self / self.length();
    }

    // Maybe optimize with the inverse multiplication or create alternate function
    #[inline]
    pub fn get_normalized_into(&self, result_out: &mut Vector3) {
        let length = self.length();
        result_out.x = self.x / length;
        result_out.y = self.y / length;
        result_out.z = self.z / length;
    }

    // Maybe change this name to just normalize
    #[inline]
    pub fn normalize(&mut self) {
        let length = self.length();
        self.x /= length;
        self.y /= length;
        self.z /= length;
    }


    #[allow(dead_code)]
    #[inline]
    pub fn mul_add(u: &Vector3, a: &Vector3, b: &Vector3) -> Vector3 {
        let x: f32 = f32::mul_add(u.x, a.x, b.x);
        let y: f32 = f32::mul_add(u.y, a.y, b.y);
        let z: f32 = f32::mul_add(u.z, a.z, b.z);

        Vector3::new(x, y, z)
    }

    #[allow(dead_code)]
    #[inline]
    pub fn scalar_mul_add(u: &Vector3, a: f32, b: &Vector3) -> Vector3 {
        let x: f32 = f32::mul_add(u.x, a, b.x);
        let y: f32 = f32::mul_add(u.y, a, b.y);
        let z: f32 = f32::mul_add(u.z, a, b.z);

        Vector3::new(x, y, z)
    }

    #[allow(dead_code)]
    #[inline]
    pub fn mul_scalar_add(u: &Vector3, a: &Vector3, b: f32) -> Vector3 {
        let x: f32 = f32::mul_add(u.x, a.x, b);
        let y: f32 = f32::mul_add(u.y, a.y, b);
        let z: f32 = f32::mul_add(u.z, a.z, b);

        Vector3::new(x, y, z)
    }

    #[allow(dead_code)]
    #[inline]
    pub fn scalar_mul_scalar_add(u: &Vector3, a: f32, b: f32) -> Vector3 {
        let x: f32 = f32::mul_add(u.x, a, b);
        let y: f32 = f32::mul_add(u.y, a, b);
        let z: f32 = f32::mul_add(u.z, a, b);

        Vector3::new(x, y, z)
    }


    #[inline]
    pub fn reflect(v: &Vector3, normal: &Vector3, reflected_out: &mut Vector3) -> bool {
        *reflected_out = (*v) - (*normal) * (2.0 * Vector3::dot(v, normal));

        true
    }

    #[inline]
    pub fn refract(v: &Vector3, n: &Vector3, etai_over_etat: f32, refracted_out: &mut Vector3) -> bool {
        let negative_uv = -*v;
        let cos_theta = Vector3::dot(&negative_uv,&n).min(1.0);
        let ray_out_perp = (*v + (*n) * cos_theta) * etai_over_etat;
        let ray_out_parallel = (-(*n)) * (1.0 - ray_out_perp.length_squared()).abs().sqrt();    
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
    pub fn random_cosine_direction(rng: &mut ThreadRng) -> Self {
        let r1 = rng.gen::<f32>();
        let r2 = rng.gen::<f32>();
        let z = (1.0 - r2).sqrt();

        let phi = 2.0 * PI * r1;
        let x = phi.cos() * r2.sqrt();
        let y = phi.sin() * r2.sqrt();
    
        Vector3::new( x, y, z )
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
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
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
            self.x = self.x - other.x;
            self.y = self.y - other.y;
            self.z = self.z - other.z;
    }
}

impl Mul<Vector3> for Vector3 {
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

impl MulAssign<f32> for Vector3 {
    #[inline]
    fn mul_assign(&mut self, factor: f32) {
        self.x = self.x * factor;
        self.y = self.y * factor;
        self.z = self.z * factor;
    }
}

impl MulAssign<Vector3> for Vector3 {
    #[inline]
    fn mul_assign(&mut self, other: Vector3) {
        self.x = self.x * other.x;
        self.y = self.y * other.y;
        self.z = self.z * other.z;
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
        self.x = self.x / factor;
        self.y = self.y / factor;
        self.z = self.z / factor;
    }
}

impl Div<Vector3> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn div(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl DivAssign<Vector3> for Vector3 {

    #[inline]
    fn div_assign(&mut self, other: Vector3) {
        self.x = self.x / other.x;
        self.y = self.y / other.y;
        self.z = self.z / other.z;
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




#[cfg(test)]
mod tests {
    use crate::math::vector3::Vector3;

    const F32_TEST_LIMIT: f32 = 0.00000000000000000000001;

    #[inline]
    fn sum(vec: Vector3) -> f32 {
        vec.x + vec.y + vec.z
    }

    #[test]
    fn test_vector3_zero() {
        let a: Vector3 = Vector3::zero();

        assert!(f32::abs(sum(a)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_new() {
        let correct: f32 = 0.6;
        let a: Vector3 = Vector3::new(0.1, 0.2, 0.3);
        let result: f32 = sum(a) - correct;

        assert!(f32::abs(result) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_add() {
        let correct: Vector3 = Vector3::new(3.3, 6.7, 4.4);

        let a: Vector3 = Vector3::new(1.0, 2.5, 3.2);
        let b: Vector3 = Vector3::new(2.3, 4.2, 1.2);
        let result: Vector3 = a + b - correct;

        assert!(f32::abs(sum(result)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_div() {
        let correct: Vector3 = Vector3::new(1.0 / 2.3, 2.5 / 4.2, 3.2 / 1.2);

        let a: Vector3 = Vector3::new(1.0, 2.5, 3.2);
        let b: Vector3 = Vector3::new(2.3, 4.2, 1.2);
        let result: Vector3 = a / b - correct;

        assert!(f32::abs(sum(result)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_length() {
        let a: Vector3 = Vector3::new(2.0, 0.0, 0.0);
        let a_correct: f32 = 2.0;
        assert!(f32::abs(a.length() - a_correct) < F32_TEST_LIMIT);

        let b: Vector3 = Vector3::new(0.0, 4.0, 0.0);
        let b_correct: f32 = 4.0;
        assert!(f32::abs(b.length() - b_correct) < F32_TEST_LIMIT);

        let c: Vector3 = Vector3::new(0.0, 0.0, 3.0);
        let c_correct: f32 = 3.0;
        assert!(f32::abs(c.length() - c_correct) < F32_TEST_LIMIT);

        let d: Vector3 = Vector3::new(1.0, 1.0, 1.0);
        let d_correct: f32 = 1.73205080757;
        assert!(f32::abs(d.length() - d_correct) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_length_squared() {
        let a: Vector3 = Vector3::new(2.0, 0.0, 0.0);
        let a_correct: f32 = 4.0;
        assert!(f32::abs(a.length_squared() - a_correct) < F32_TEST_LIMIT);

        let b: Vector3 = Vector3::new(0.0, 3.0, 0.0);
        let b_correct: f32 = 9.0;
        assert!(f32::abs(b.length_squared() - b_correct) < F32_TEST_LIMIT);

        let c: Vector3 = Vector3::new(0.0, 0.0, 4.0);
        let c_correct: f32 = 16.0;
        assert!(f32::abs(c.length_squared() - c_correct) < F32_TEST_LIMIT);

        let d: Vector3 = Vector3::new(1.1, 1.2, 1.3);
        let d_correct: f32 = 4.34000000000014222336;
        assert!(f32::abs(d.length_squared() - d_correct) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_dot() {
        let correct: f32 = 12.0;

        let a: Vector3 = Vector3::new(1.0, 2.0, 3.0);
        let b: Vector3 = Vector3::new(4.0, -5.0, 6.0);
        let result: f32 = Vector3::dot(&a, &b) - correct;

        assert!(f32::abs(result) < F32_TEST_LIMIT);
    }
    
    #[test]
    fn test_vector3_cross() {
        let correct: Vector3 = Vector3::new(-1.0, -38.0, -16.0);

        let a: Vector3 = Vector3::new(4.0, 2.0, -5.0);
        let b: Vector3 = Vector3::new(2.0, -3.0, 7.0);
        let result: Vector3 = Vector3::cross(&a, &b) - correct;

        assert!(f32::abs(sum(result)) < F32_TEST_LIMIT);
    }
    
    #[test]
    fn test_vector3_cross_into() {
        let correct: Vector3 = Vector3::new(-1.0, -38.0, -16.0);

        let a: Vector3 = Vector3::new(4.0, 2.0, -5.0);
        let b: Vector3 = Vector3::new(2.0, -3.0, 7.0);

        let mut result: Vector3 = Vector3::zero();
        Vector3::cross_into(&a, &b, &mut result);
        let result: Vector3 = result - correct;

        assert!(f32::abs(sum(result)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_normalized() {
        let correct_scalar: f32 = 0.57735026918925152901829780358145;
        let correct: Vector3 = Vector3::new(correct_scalar, correct_scalar, correct_scalar);
        let a: Vector3 = Vector3::new(1.0, 1.0, 1.0);
        let result: Vector3 = a.get_normalized() - correct;

        assert!(f32::abs(sum(result)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_normalized_into() {
        let correct_scalar: f32 = 0.57735026918925152901829780358145;
        let correct: Vector3 = Vector3::new(correct_scalar, correct_scalar, correct_scalar);
        let a: Vector3 = Vector3::new(1.0, 1.0, 1.0);
        let mut result: Vector3 = Vector3::zero();
        a.get_normalized_into(&mut result);
        let result: Vector3 = result - correct;

        assert!(f32::abs(sum(result)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_normalize() {
        let correct_scalar: f32 = 0.57735026918925152901829780358145;
        let correct: Vector3 = Vector3::new(correct_scalar, correct_scalar, correct_scalar);
        let mut a: Vector3 = Vector3::new(1.0, 1.0, 1.0);
        a.normalize();
        let result: Vector3 = a - correct;

        assert!(f32::abs(sum(result)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_reflect() {
        let correct: Vector3 = Vector3::new(-2925.56, -5118.34, -465.76);

        let v: Vector3 = Vector3::new(1.0, 3.14, 22.0);
        let normal: Vector3 = Vector3::new(12.0, 21.0, 2.0);
        let mut reflected_out: Vector3 = Vector3::zero();

        Vector3::reflect(&v, &normal, &mut reflected_out);

        let result: Vector3 = reflected_out - correct;

        assert!(f32::abs(sum(result)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_refract() {
        let correct: Vector3 = Vector3::new(-115915.484, -202847.72, -19250.69);
        
        let v: Vector3 = Vector3::new(1.0, 3.14, 22.0);
        let n: Vector3 = Vector3::new(12.0, 21.0, 2.0);
        let etai_over_etat: f32 = 3.14;
        let mut refracted_out: Vector3 = Vector3::zero();

        Vector3::refract(&v, &n, etai_over_etat, &mut refracted_out);
        
        let result: Vector3 = refracted_out - correct;

        assert!(f32::abs(sum(result)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_index() {
        let a: Vector3 = Vector3::new(1.0 / 2.3, 2.5 / 4.2, 3.2 / 1.2);

        assert!(a.x == a[0] && a.y == a[1] && a.z == a[2]);
    }

    #[test]
    fn test_vector3_mul_add() {
        let a: Vector3 = Vector3::new(13.0 / 2.3, 2.5 / 4.2, 3.2 / 1.2);
        let b: Vector3 = Vector3::new(17.0 / 1.3, 2.5 / 4.2, 3.2 / 1.2);
        let c: Vector3 = Vector3::new(11.0 / 6.3, 24.5 / 4.2, 32.2 / 1.2);
        let d: Vector3 = Vector3::new(112.0 / 2.3, 245.5 / 4.2, 313.2 / 1.2);

        let result: Vector3 = Vector3::mul_add(&a, &b, &Vector3::mul_add(&c, &d, &b));
        let correct: Vector3 = Vector3::new(172.01411, 341.9218, 7013.278);

        let result: Vector3 = result - correct;

        assert!(f32::abs(result.x) < F32_TEST_LIMIT && f32::abs(result.y) < F32_TEST_LIMIT && f32::abs(result.z) < F32_TEST_LIMIT);
        assert!(f32::abs(sum(result)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_scalar_mul_add() {
        let a: Vector3 = Vector3::new(13.0 / 2.3, 2.5 / 4.2, 3.2 / 1.2);
        let b: f32 = 3.14;
        let c: Vector3 = Vector3::new(11.0 / 6.3, 24.5 / 4.2, 32.2 / 1.2);
        let d: f32 = -2.145;

        let result: Vector3 = Vector3::scalar_mul_add(&a, b, &Vector3::scalar_mul_add(&c, d, &a));
        let correct: Vector3 = Vector3::new(19.654762, -10.048214, -46.517498);

        let result: Vector3 = result - correct;

        assert!(f32::abs(result.x) < F32_TEST_LIMIT && f32::abs(result.y) < F32_TEST_LIMIT && f32::abs(result.z) < F32_TEST_LIMIT);
        assert!(f32::abs(sum(result)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_mul_scalar_add() {
        let a: Vector3 = Vector3::new(-13.0 / 2.3, 2.5 / 4.2, -3.2 / 1.2);
        let b: f32 = 3.14;
        let c: Vector3 = Vector3::new(11.0 / 6.3, -24.5 / 4.2, 32.2 / 1.2);
        let d: f32 = -2.145;

        let result: Vector3 = Vector3::mul_scalar_add(&a, &Vector3::mul_scalar_add(&c, &a, d), b);
        let correct: Vector3 = Vector3::new(71.04451, -0.2035852, 199.67477);

        let result: Vector3 = result - correct;

        assert!(f32::abs(result.x) < F32_TEST_LIMIT && f32::abs(result.y) < F32_TEST_LIMIT && f32::abs(result.z) < F32_TEST_LIMIT);
        assert!(f32::abs(sum(result)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_scalar_mul_scalar_add() {
        let a: Vector3 = Vector3::new(-13.0 / 2.3, 2.5 / 4.2, -3.2 / 1.2);
        let b: f32 = 3.14;
        let c: f32 = -2.145;

        let result: Vector3 = Vector3::scalar_mul_scalar_add(&a, b, c);
        let correct: Vector3 = Vector3::new(-19.892826, -0.27595213, -10.518333);

        let result: Vector3 = result - correct;

        assert!(f32::abs(result.x) < F32_TEST_LIMIT && f32::abs(result.y) < F32_TEST_LIMIT && f32::abs(result.z) < F32_TEST_LIMIT);
        assert!(f32::abs(sum(result)) < F32_TEST_LIMIT);
    }

}