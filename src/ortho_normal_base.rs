use std::ops::{Index, IndexMut};

use crate::vector3::Vector3;

pub struct OrthoNormalBase {
    pub u: Vector3,
    pub v: Vector3,
    pub w: Vector3,
}

impl OrthoNormalBase {
    pub fn build_from_w(n: &Vector3) -> OrthoNormalBase {
        let w = n.normalized();
        let a = if 0.9 < w.x.abs() { Vector3{x: 0.0, y: 1.0, z: 0.0 } } else { Vector3{x: 1.0, y: 0.0, z: 0.0 }};
        let v = Vector3::cross(&w, &a).normalized();
        let u = Vector3::cross(&w, &v);

        OrthoNormalBase { u, v, w }
    }

    pub fn local_vector(&self, a: &Vector3) -> Vector3 {
        a.x * self.u + a.y * self.v + a.z * self.w
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