use std::ops::{Index, IndexMut};

use ultraviolet::{Vec3};


const X_VECTOR : Vec3 = Vec3::new( 1.0, 0.0, 0.0);
const Y_VECTOR : Vec3 = Vec3::new( 0.0, 1.0, 0.0);

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

    // Update this to not require any more new structs. Maybe do a cross and normalized with a supplied result argument
    // And make the two Vector3's static
    #[inline]
    pub fn update(&mut self, n: Vec3) {
        self.w = n;
        self.w.normalize();
        let a: &Vec3 = if 0.9 < self.w.x.abs() { &Y_VECTOR } else { &X_VECTOR };
        self.v = self.w.cross(*a);
        self.v.normalize();
        self.u = self.w.cross(self.v);
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