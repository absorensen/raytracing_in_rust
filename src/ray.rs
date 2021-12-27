use crate::vector3::Vector3;


#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin : Vector3,
    pub direction: Vector3,
}

impl Ray {

    pub fn new (origin: Vector3, direction: Vector3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t:f64) -> Vector3 {
        self.origin + t * self.direction
    }

    pub fn reflect(v: &Vector3, normal: &Vector3) -> Vector3 {
        *v - 2.0 * Vector3::dot(v, normal) * *normal
    }


}