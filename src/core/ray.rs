use nalgebra::Vector3;

#[derive(Copy, Clone, Debug, Default)]
pub struct Ray {
    pub origin : Vector3<f32>,
    pub direction: Vector3<f32>,
    pub time: f32,
}

impl Ray {

    #[allow(dead_code)]
    #[inline]
    pub fn new (origin: Vector3<f32>, direction: Vector3<f32>, time: f32) -> Self {
        Ray { origin, direction, time }
    }

    #[inline]
    pub fn new_normalized (origin: Vector3<f32>, direction: Vector3<f32>, time: f32) -> Self {
        let mut ray: Ray = Ray { origin, direction, time };
        
        ray.direction.normalize();

        ray
    }

    #[inline]
    pub fn at(&self, t:f32) -> Vector3<f32> {
        self.origin + self.direction * t
    }

}