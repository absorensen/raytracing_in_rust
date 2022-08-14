use crate::math::vector3::Vector3;

#[derive(Copy, Clone, Debug, Default)]
pub struct Ray {
    pub origin : Vector3,
    pub direction: Vector3,
    pub time: f32,
}

impl Ray {

    #[allow(dead_code)]
    #[inline]
    pub fn new (origin: Vector3, direction: Vector3, time: f32) -> Self {
        Ray { origin, direction, time }
    }

    #[inline]
    pub fn new_normalized (origin: Vector3, direction: Vector3, time: f32) -> Self {
        let mut ray: Ray = Ray { origin, direction, time };
        
        ray.direction.normalize();

        ray
    }

    #[inline]
    pub fn at(&self, t:f32) -> Vector3 {
        self.origin + self.direction * t
    }

}


#[cfg(test)]
mod tests {
    use crate::{math::vector3::Vector3, core::ray::Ray};

    const F32_TEST_LIMIT: f32 = 0.00000000000000000000001;

    #[test]
    fn test_ray_new_normalized() {
        let origin : Vector3 = Vector3::new(-5000.0, 3.14, -1623.0);
        let direction : Vector3 = Vector3::new(-5000.0, 3.14, -1623.0);
        let time : f32 = 3.14;

        let a: Ray = Ray::new_normalized(origin, direction, time);

        assert!(a.origin.x - (-5000.0) < F32_TEST_LIMIT);
        assert!(a.origin.y - 3.14 < F32_TEST_LIMIT);
        assert!(a.origin.z - (-1623.0) < F32_TEST_LIMIT);

        assert!(f32::abs(a.direction.x - (-0.95114565)) < F32_TEST_LIMIT);
        assert!(f32::abs(a.direction.y - 0.0005973195) < F32_TEST_LIMIT);
        assert!(f32::abs(a.direction.z - (-0.3087419)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_ray_new_unnormalized() {
        let origin : Vector3 = Vector3::new(-5000.0, 3.14, -1623.0);
        let direction : Vector3 = Vector3::new(-1230.0, 30.14, 132623.0);
        let time : f32 = 3.14;

        let a: Ray = Ray::new(origin, direction, time);

        assert!(a.origin.x - (-5000.0) < F32_TEST_LIMIT);
        assert!(a.origin.y - 3.14 < F32_TEST_LIMIT);
        assert!(a.origin.z - (-1623.0) < F32_TEST_LIMIT);

        assert!(f32::abs(a.direction.x - (-1230.0)) < F32_TEST_LIMIT);
        assert!(f32::abs(a.direction.y - 30.14) < F32_TEST_LIMIT);
        assert!(f32::abs(a.direction.z - 132623.0) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_ray_at() {
        let origin : Vector3 = Vector3::new(-5000.0, 3.14, -1623.0);
        let direction : Vector3 = Vector3::new(-5000.0, 3.14, -1623.0);
        let time : f32 = 3.14;

        let a: Ray = Ray::new_normalized(origin, direction, time);

        let at: Vector3 = a.at(time / 3.0);

        assert!(f32::abs(at.x - (-5000.9956)) < F32_TEST_LIMIT);
        assert!(f32::abs(at.y - 3.1406252) < F32_TEST_LIMIT);
        assert!(f32::abs(at.z -(-1623.3231)) < F32_TEST_LIMIT);

    }

}