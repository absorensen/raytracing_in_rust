use std::ops::{Index, IndexMut};

use nalgebra::Vector3;


const X_VECTOR : Vector3<f32> = Vector3::<f32>::new( 1.0, 0.0, 0.0);
const Y_VECTOR : Vector3<f32> = Vector3::<f32>::new( 0.0, 1.0, 0.0);

#[derive(Clone, Copy)]
pub struct OrthoNormalBase {
    pub u: Vector3<f32>,
    pub v: Vector3<f32>,
    pub w: Vector3<f32>,
}

impl OrthoNormalBase {
    pub fn build_from_w(n: &Vector3<f32>) -> OrthoNormalBase {
        let w: Vector3<f32> = n.normalize();
        let a: Vector3<f32> = if 0.9 < w.x.abs() { Vector3::<f32>::new(0.0, 1.0, 0.0 ) } else { Vector3::<f32>::new(1.0, 0.0, 0.0 )};
        let v: Vector3<f32> = Vector3::cross(&w, &a).normalize();
        let u: Vector3<f32> = Vector3::cross(&w, &v);

        OrthoNormalBase { u, v, w }
    }

    // Update this to not require any more new structs. Maybe do a cross and normalized with a supplied result argument
    // And make the two Vector3's static
    #[inline]
    pub fn update(&mut self, n: Vector3<f32>) {
        self.w = n;
        self.w.normalize();
        let a: &Vector3<f32> = if 0.9 < self.w.x.abs() { &Y_VECTOR } else { &X_VECTOR };
        self.v = Vector3::cross(&self.w, &a);
        self.v.normalize();
        self.u = Vector3::cross(&self.w, &self.v);
    }

    pub fn local_vector(&self, a: &Vector3<f32>) -> Vector3<f32> {
        self.u * a.x + &self.v * a.y + self.w * a.z
    }
}

impl Index<usize> for OrthoNormalBase {
    type Output = Vector3<f32>;
    #[inline]
    fn index(&self, i: usize) -> &Vector3<f32> {
        match i {
            0 => &self.u,
            1 => &self.v,
            2 => &self.w,
            _ => unreachable!(),
        }
    }
}

impl IndexMut<usize> for OrthoNormalBase {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Vector3<f32> {
        match i {
            0 => &mut self.u,
            1 => &mut self.v,
            2 => &mut self.w,
            _ => unreachable!(),
        }
    }
}