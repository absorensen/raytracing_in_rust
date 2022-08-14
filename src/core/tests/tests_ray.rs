#[cfg(test)]
mod tests {
    use crate::{math::vector3::Vector3, core::ray::Ray};

    const F32_TEST_LIMIT: f32 = f32::EPSILON;

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