use std::{ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg, Index, IndexMut}, iter::Sum, fmt};

use minifb::clamp;
use rand::{rngs::{ThreadRng, mock::StepRng}, Rng};
use rand_chacha::ChaChaRng;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct ColorRGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl ColorRGB {
    #[inline]
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    #[inline]
    pub fn black() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    #[inline]
    pub fn white() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    #[inline]
    pub fn is_nan(&self) -> bool {
        self.r.is_nan() || self.g.is_nan() || self.b.is_nan() 
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
    pub fn mul_add_inplace(&mut self, mul: &ColorRGB, add: &ColorRGB) {
        self.r = f32::mul_add(self.r, mul.r, add.r);
        self.g = f32::mul_add(self.g, mul.g, add.g);
        self.b = f32::mul_add(self.b, mul.b, add.b);

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

    #[inline]
    pub fn scale_for_output(&mut self, scale: f32) {
        if self.r.is_nan() { self.r = 0.0; }
        if self.g.is_nan() { self.g = 0.0; }
        if self.b.is_nan() { self.b = 0.0; }

        // Try and apply this scaling to the colors before summation
        self.r = 255.999 * clamp(0.0, (scale * self.r).sqrt(), 0.999);
        self.g = 255.999 * clamp(0.0, (scale * self.g).sqrt(), 0.999);
        self.b = 255.999 * clamp(0.0, (scale * self.b).sqrt(), 0.999);
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

impl MulAssign<ColorRGB> for ColorRGB {

    #[inline]
    fn mul_assign(&mut self, other: ColorRGB) {
            self.r = self.r * other.r;
            self.g = self.g * other.g;
            self.b = self.b * other.b;
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
    #[inline]
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
    #[inline]
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