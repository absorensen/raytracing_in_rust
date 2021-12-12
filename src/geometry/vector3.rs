#[derive(Copy, Clone, Default, Debug)]
pub struct Vector3(pub f64, pub f64, pub f64);

impl Vector3 {
    #[inline]
    pub fn dot(&self, other: Self) -> f64 {
        self.zip_with(other, core::ops::Mul::mul)
            .reduce(core::ops::Add::add)
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vector3(
            self.1 * other.2 - self.2 * other.1,
            -(self.0 * other.2 - self.2 * other.0),
            self.0 * other.1 - self.1 * other.0,
        )
    }

    #[inline]
    pub fn length(&self) -> f64 {
        self.dot(*self).sqrt()
    }

    #[inline]
    pub fn unit_vector(&self) -> Vector3 {
        *self / self.length()
    }

    #[inline]
    pub fn length_squared(&self) -> f64 {
        let length = self.length();
        length * length
    }


    #[inline]
    pub fn zip_with(self, other: Vector3, mut f: impl FnMut(f64, f64) -> f64) -> Self {
        Vector3(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2))
    }

    #[inline]
    pub fn reduce(self, f: impl Fn(f64, f64) -> f64) -> f64 {
        f(f(self.0, self.1), self.2)
    }

    #[inline]
    pub fn map(self, mut f: impl FnMut(f64) -> f64) -> Self {
        Vector3(f(self.0), f(self.1), f(self.2))
    }
    
    #[inline]
    pub fn color_to_output(self, image_buffer: &mut Vec<f64>, offset: usize) -> () {
        image_buffer[offset + 0] = (255.999 * self.0) as f64;
        image_buffer[offset + 1] = (255.999 * self.1) as f64;
        image_buffer[offset + 2] = (255.999 * self.2) as f64;
    }

}

impl From<f64> for Vector3 {
    #[inline]
    fn from(v: f64) -> Self {
        Vector3(v, v, v)
    }
} 

impl std::ops::Mul for Vector3 {
    type Output = Vector3;

    #[inline]
    fn mul(self, rhs: Vector3) -> Self::Output {
        self.zip_with(rhs, std::ops::Mul::mul)
    }
}

impl std::ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    #[inline]
    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3::from(self) * rhs
    }
}

impl std::ops::Div for Vector3 {
    type Output = Vector3;

    #[inline]
    fn div(self, rhs: Vector3) -> Self::Output {
        self.zip_with(rhs, std::ops::Div::div)
    }
}

impl std::ops::Div<f64> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        self.map(|x| x / rhs)
    }
}

impl std::ops::Add for Vector3 {
    type Output = Vector3;

    #[inline]
    fn add(self, rhs: Vector3) -> Self::Output {
        self.zip_with(rhs, std::ops::Add::add)
    }
}

impl std::ops::Add<Vector3> for f64 {
    type Output = Vector3;

    #[inline]
    fn add(self, rhs: Vector3) -> Self::Output {
        rhs.map(|x| self + x)
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Vector3;

    #[inline]
    fn sub(self, rhs: Vector3) -> Self::Output {
        self.zip_with(rhs, std::ops::Sub::sub)
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Vector3;

    #[inline]
    fn neg(self) -> Self::Output {
        self.map(std::ops::Neg::neg)
    }
}

/// Allow accumulation of vectors from an iterator.
impl std::iter::Sum for Vector3 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Vector3::default(), std::ops::Add::add)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Channel {
    R,
    G,
    B,
}

use Channel::*;

impl ::std::ops::Index<Channel> for Vector3 {
    type Output = f64;

    #[inline]
    fn index(&self, index: Channel) -> &Self::Output {
        match index {
            R => &self.0,
            G => &self.1,
            B => &self.2,
        }
    }
}

impl ::std::ops::IndexMut<Channel> for Vector3 {
    #[inline]
    fn index_mut(&mut self, index: Channel) -> &mut Self::Output {
        match index {
            R => &mut self.0,
            G => &mut self.1,
            B => &mut self.2,
        }
    }
}

