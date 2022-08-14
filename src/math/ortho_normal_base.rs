use std::ops::{Index, IndexMut};

use super::vector3::Vector3;

const X_VECTOR : Vector3 = Vector3{x: 1.0, y: 0.0, z: 0.0};
const Y_VECTOR : Vector3 = Vector3{x: 0.0, y: 1.0, z: 0.0};

pub struct OrthoNormalBase {
    pub u: Vector3,
    pub v: Vector3,
    pub w: Vector3,
}

impl OrthoNormalBase {
    pub fn build_from_w(n: &Vector3) -> OrthoNormalBase {
        let w = n.get_normalized();
        let a = if 0.9 < w.x.abs() { Vector3{x: 0.0, y: 1.0, z: 0.0 } } else { Vector3{x: 1.0, y: 0.0, z: 0.0 }};
        let v = Vector3::cross(&w, &a).get_normalized();
        let u = Vector3::cross(&w, &v);

        OrthoNormalBase { u, v, w }
    }

    // Update this to not require any more new structs. Maybe do a cross and normalized with a supplied result argument
    // And make the two Vector3's static
    #[inline]
    pub fn update(&mut self, n: Vector3) {
        self.w = n;
        self.w.normalize();
        let a: &Vector3 = if 0.9 < self.w.x.abs() { &Y_VECTOR } else { &X_VECTOR };
        Vector3::cross_into(&self.w, &a, &mut self.v);
        self.v.normalize();
        Vector3::cross_into(&self.w, &self.v, &mut self.u);
    }

    pub fn local_vector(&self, a: &Vector3) -> Vector3 {
        Vector3::scalar_mul_add(&self.u, a.x, &Vector3::scalar_mul_add(&self.v, a.y, &(self.w * a.z)))
    }
}

impl Index<usize> for OrthoNormalBase {
    type Output = Vector3;

    fn index(&self, i: usize) -> &Vector3 {
        match i {
            0 => &self.u,
            1 => &self.v,
            2 => &self.w,
            _ => unreachable!(),
        }
    }
}

impl IndexMut<usize> for OrthoNormalBase {
    fn index_mut(&mut self, i: usize) -> &mut Vector3 {
        match i {
            0 => &mut self.u,
            1 => &mut self.v,
            2 => &mut self.w,
            _ => unreachable!(),
        }
    }
}