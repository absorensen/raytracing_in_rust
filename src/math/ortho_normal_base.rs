use std::ops::{Index, IndexMut};

use ultraviolet::{Vec3};

#[derive(Clone, Copy)]
pub struct OrthoNormalBase {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl OrthoNormalBase {
    pub fn build_from_w(n: &Vec3) -> OrthoNormalBase {
        let w: Vec3 = n.normalized();
        let a: Vec3 = if 0.9 < w.x.abs() { Vec3::new(0.0, 1.0, 0.0 ) } else { Vec3::new(1.0, 0.0, 0.0 )};
        let mut v: Vec3 = w.cross(a);
        v.normalize();
        let u: Vec3 = w.cross(v);

        OrthoNormalBase { u, v, w }
    }

    pub fn local_vector(&self, a: &Vec3) -> Vec3 {
        self.u * a.x + self.v * a.y + self.w * a.z
    }
}

impl Index<usize> for OrthoNormalBase {
    type Output = Vec3;
    #[inline]
    fn index(&self, i: usize) -> &Vec3 {
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
    fn index_mut(&mut self, i: usize) -> &mut Vec3 {
        match i {
            0 => &mut self.u,
            1 => &mut self.v,
            2 => &mut self.w,
            _ => unreachable!(),
        }
    }
}