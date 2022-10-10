use ultraviolet::Vec3;

#[derive(Copy, Clone, Debug, Default)]
pub struct Ray {
    pub origin : Vec3,
    pub direction: Vec3,
    pub time: f32,
}

impl Ray {

    #[allow(dead_code)]
    #[inline]
    pub fn new (origin: Vec3, direction: Vec3, time: f32) -> Self {
        Ray { origin, direction, time }
    }

    #[inline]
    pub fn new_normalized (origin: Vec3, direction: Vec3, time: f32) -> Self {
        let mut ray: Ray = Ray { origin, direction, time };
        
        ray.direction.normalize();

        ray
    }

    #[inline]
    pub fn at(&self, t:f32) -> Vec3 {
        self.origin + self.direction * t
    }

}