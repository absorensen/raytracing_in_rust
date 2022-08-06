use crate::vector3::Vector3;


#[derive(Copy, Clone, Debug, Default)]
pub struct Ray {
    pub origin : Vector3,
    pub direction: Vector3,
    pub time: f32,
}

impl Ray {

    pub fn new (origin: Vector3, direction: Vector3, time: f32) -> Self {
        Ray { origin, direction, time }
    }

    #[inline]
    pub fn at(&self, t:f32) -> Vector3 {
        self.origin + t * self.direction
    }

}