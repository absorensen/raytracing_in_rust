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
        // Implementation of Frisvad based on onb code from https://people.compute.dtu.dk/jerf/code/
        if(n.z < -0.999999999) {
            let v: Vector3<f32> = Vector3::<f32>::new( 0.0, -1.0, 0.0);
            let u: Vector3<f32> = Vector3::<f32>::new(-1.0,  0.0, 0.0);
            return OrthoNormalBase { u, v, w: n.clone() };
        }

        let a: f32 = 1.0/(1.0 + n.z);
        let b: f32 = -n.x*n.y*a;
        let v: Vector3<f32> = Vector3::<f32>::new(1.0 - n.x*n.x*a, b, -n.x);
        let u: Vector3<f32> = Vector3::<f32>::new(b, 1.0 - n.y*n.y*a, -n.y);
        return OrthoNormalBase { u, v, w: n.clone() };

    }

    // Update this to not require any more new structs. Maybe do a cross and normalized with a supplied result argument
    // And make the two Vector3's static
    #[inline]
    pub fn update(&mut self, n: Vector3<f32>) {
        // Implementation of Frisvad based on onb code from https://people.compute.dtu.dk/jerf/code/
        self.w = n;
        if self.w.z < -0.999999999 {
            self.v.x = 0.0;
            self.v.y = -1.0;
            self.v.z = 0.0;

            self.u.x = -1.0;
            self.u.y = 0.0;
            self.u.z = 0.0;
            return;
        }

        let a: f32 = 1.0/(1.0 + n.z);
        let b: f32 = -n.x*n.y*a;

        self.v.x = 1.0 - n.x*n.x*a;
        self.v.y = b;
        self.v.z = -n.x;
        
        self.u.x = b;
        self.u.y = 1.0 - n.y*n.y*a;
        self.u.z = -n.y;
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