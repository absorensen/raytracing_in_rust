use std::{ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg, Index, IndexMut}, iter::Sum, fmt};

use rand::{rngs::{ThreadRng, mock::StepRng}, Rng};
use rand_chacha::ChaChaRng;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct ColorRGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl ColorRGB {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn black() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn white() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }


    #[inline]
    pub fn random(rng: &mut ThreadRng) -> Self {
        ColorRGB { r: rng.gen::<f32>(), g: rng.gen::<f32>(), b: rng.gen::<f32>() }
    }

    #[inline]
    pub fn random_range(rng: &mut ThreadRng, minimum: f32, maximum: f32) -> Self {
        ColorRGB { r: rng.gen_range(minimum..maximum), g: rng.gen_range(minimum..maximum), b: rng.gen_range(minimum..maximum) }
    }

    #[inline]
    pub fn random_chacha(rng: &mut ChaChaRng) -> Self {
        ColorRGB { r: rng.gen::<f32>(), g: rng.gen::<f32>(), b: rng.gen::<f32>() }
    }

    #[inline]
    pub fn random_range_chacha(rng: &mut ChaChaRng, minimum: f32, maximum: f32) -> Self {
        ColorRGB { r: rng.gen_range(minimum..maximum), g: rng.gen_range(minimum..maximum), b: rng.gen_range(minimum..maximum) }
    }

    #[inline]
    pub fn random_step(rng: &mut StepRng) -> Self {
        ColorRGB { r: rng.gen::<f32>(), g: rng.gen::<f32>(), b: rng.gen::<f32>() }
    }

    #[inline]
    pub fn random_range_step(rng: &mut StepRng, minimum: f32, maximum: f32) -> Self {
        ColorRGB { r: rng.gen_range(minimum..maximum), g: rng.gen_range(minimum..maximum), b: rng.gen_range(minimum..maximum) }
    }

}

impl Add for ColorRGB {
    type Output = ColorRGB;

    #[inline]
    fn add(self, rhs: ColorRGB) -> ColorRGB {
        ColorRGB {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl AddAssign for ColorRGB {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.r = self.r + other.r;
        self.g = self.g + other.g;
        self.b = self.b + other.b;
    }
}

impl Sub for ColorRGB {
    type Output = ColorRGB;

    #[inline]
    fn sub(self, rhs: ColorRGB) -> ColorRGB {
        ColorRGB {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl SubAssign for ColorRGB {

    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.r = self.r - other.r;
        self.g = self.g - other.g;
        self.b = self.b - other.b;
    }
}

impl Mul for ColorRGB {
    type Output = ColorRGB;

    #[inline]
    fn mul(self, rhs: ColorRGB) -> ColorRGB {
        ColorRGB {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Mul<f32> for ColorRGB {
    type Output = ColorRGB;

    #[inline]
    fn mul(self, factor: f32) -> ColorRGB {
        ColorRGB {
            r: self.r * factor,
            g: self.g * factor,
            b: self.b * factor,
        }
    }
}

impl Mul<ColorRGB> for f32 {
    type Output = ColorRGB;

    #[inline]
    fn mul(self, other: ColorRGB) -> ColorRGB {
        ColorRGB {
            r: self * other.r,
            g: self * other.g,
            b: self * other.b,
        }
    }
}

impl MulAssign<f32> for ColorRGB {

    #[inline]
    fn mul_assign(&mut self, factor: f32) {
            self.r = self.r * factor;
            self.g = self.g * factor;
            self.b = self.b * factor;
    }
}

impl Div<f32> for ColorRGB {
    type Output = ColorRGB;

    #[inline]
    fn div(self, factor: f32) -> ColorRGB {
        ColorRGB {
            r: self.r / factor,
            g: self.g / factor,
            b: self.b / factor,
        }
    }
}

impl DivAssign<f32> for ColorRGB {

    #[inline]
    fn div_assign(&mut self, factor: f32) {
        self.r = self.r / factor;
        self.g = self.g / factor;
        self.b = self.b / factor;
    }
}

impl Div<ColorRGB> for ColorRGB {
    type Output = ColorRGB;

    #[inline]
    fn div(self, other: ColorRGB) -> ColorRGB {
        ColorRGB {
            r: self.r / other.r,
            g: self.g / other.g,
            b: self.b / other.b,
        }
    }
}

impl DivAssign<ColorRGB> for ColorRGB {
    #[inline]
    fn div_assign(&mut self, other: ColorRGB) {
        self.r = self.r / other.r;
        self.g = self.g / other.g;
        self.b = self.b / other.b;
    }
}

impl Neg for ColorRGB {
    type Output = ColorRGB;

    #[inline]
    fn neg(self) -> ColorRGB {
        ColorRGB {
            r: -self.r,
            g: -self.g,
            b: -self.b,
        }
    }
}

impl Sum for ColorRGB {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self { r: 0.0, g: 0.0, b: 0.0}, |a, b| Self {
            r: a.r + b.r,
            g: a.g + b.g,
            b: a.b + b.b,
        })
    }
}

impl Index<usize> for ColorRGB {
    type Output = f32;

    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _ => unreachable!(),
        }
    }
}

impl IndexMut<usize> for ColorRGB {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        match i {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for ColorRGB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }
}



#[cfg(test)]
mod tests {
    use crate::core::color_rgb::{ColorRGB};

    const F32_TEST_LIMIT: f32 = 0.00000000000000000000001;

    #[inline]
    fn sum(color: &ColorRGB) -> f32 {
        color.r + color.g + color.b
    }

    #[inline]
    fn approximately_equal(color:&ColorRGB, reference: &ColorRGB) -> bool {
        let difference: ColorRGB = *color - *reference;

        f32::abs(difference.r) < F32_TEST_LIMIT && f32::abs(difference.g) < F32_TEST_LIMIT && f32::abs(difference.b) < F32_TEST_LIMIT
    }

    #[test]
    fn test_color_rgb_black() {
        let a: ColorRGB = ColorRGB::black();

        assert!(a.r < F32_TEST_LIMIT && a.g < F32_TEST_LIMIT && a.b < F32_TEST_LIMIT);
    }

    #[test]
    fn test_color_rgb_white() {
        let a: ColorRGB = ColorRGB::white();

        assert!((a.r - 1.0) < F32_TEST_LIMIT && (a.g - 1.0) < F32_TEST_LIMIT && (a.b - 1.0) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_color_rgb_new() {
        let correct_scalar: f32 = 0.6;
        let correct_struct: ColorRGB = ColorRGB{r:0.1, g: 0.2, b: 0.3};
        
        let a: ColorRGB = ColorRGB::new(0.1, 0.2, 0.3);

        let result: f32 = sum(&a) - correct_scalar;

        assert!(f32::abs(result) < F32_TEST_LIMIT);
        assert!(approximately_equal(&a, &correct_struct))
    }

    #[test]
    fn test_color_rgb_random() {
        let upper_limit: f32 = 1.0;
        let lower_limit: f32 = 0.0;

        let iteration_count: usize = 1000;
        
        let mut rng = rand::thread_rng();

        for _iteration_index in 0..iteration_count {
            let a: ColorRGB = ColorRGB::random(&mut rng);

            assert!(
                lower_limit <= a.r && 
                a.r < upper_limit && 
                lower_limit <= a.g &&
                a.g < upper_limit &&
                lower_limit <= a.b &&
                a.b < upper_limit
            );
        }
    }

    #[test]
    fn test_color_rgb_add() {
        let correct: ColorRGB = ColorRGB::new(3.3, 6.7, 4.4);

        let a: ColorRGB = ColorRGB::new(1.0, 2.5, 3.2);
        let b: ColorRGB = ColorRGB::new(2.3, 4.2, 1.2);
        let result: ColorRGB = a + b - correct;

        assert!(f32::abs(sum(&result)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_color_rgb_div() {
        let correct: ColorRGB = ColorRGB::new(1.0 / 2.3, 2.5 / 4.2, 3.2 / 1.2);

        let a: ColorRGB = ColorRGB::new(1.0, 2.5, 3.2);
        let b: ColorRGB = ColorRGB::new(2.3, 4.2, 1.2);
        let result: ColorRGB = a / b - correct;

        assert!(f32::abs(sum(&result)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_color_rgb_index() {
        let a: ColorRGB = ColorRGB::new(1.0 / 2.3, 2.5 / 4.2, 3.2 / 1.2);

        assert!(a.r == a[0] && a.g == a[1] && a.b == a[2]);
    }

}