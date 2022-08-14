#[cfg(test)]
mod tests {
    use crate::math::vector3::Vector3;

    const F32_TEST_LIMIT: f32 = f32::EPSILON;

    #[inline]
    fn sum(vec: Vector3) -> f32 {
        vec.x + vec.y + vec.z
    }

    #[test]
    fn test_vector3_zero() {
        let a: Vector3 = Vector3::zero();

        assert!(f32::abs(sum(a)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_new() {
        let correct: f32 = 0.6;
        let a: Vector3 = Vector3::new(0.1, 0.2, 0.3);
        let result: f32 = sum(a) - correct;

        assert!(f32::abs(result) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_add() {
        let correct: Vector3 = Vector3::new(3.3, 6.7, 4.4);

        let a: Vector3 = Vector3::new(1.0, 2.5, 3.2);
        let b: Vector3 = Vector3::new(2.3, 4.2, 1.2);
        let result: Vector3 = a + b - correct;

        assert!(f32::abs(sum(result)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_div() {
        let correct: Vector3 = Vector3::new(1.0 / 2.3, 2.5 / 4.2, 3.2 / 1.2);

        let a: Vector3 = Vector3::new(1.0, 2.5, 3.2);
        let b: Vector3 = Vector3::new(2.3, 4.2, 1.2);
        let result: Vector3 = a / b - correct;

        assert!(f32::abs(sum(result)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_length() {
        let a: Vector3 = Vector3::new(2.0, 0.0, 0.0);
        let a_correct: f32 = 2.0;
        assert!(f32::abs(a.length() - a_correct) < F32_TEST_LIMIT);

        let b: Vector3 = Vector3::new(0.0, 4.0, 0.0);
        let b_correct: f32 = 4.0;
        assert!(f32::abs(b.length() - b_correct) < F32_TEST_LIMIT);

        let c: Vector3 = Vector3::new(0.0, 0.0, 3.0);
        let c_correct: f32 = 3.0;
        assert!(f32::abs(c.length() - c_correct) < F32_TEST_LIMIT);

        let d: Vector3 = Vector3::new(1.0, 1.0, 1.0);
        let d_correct: f32 = 1.73205080757;
        assert!(f32::abs(d.length() - d_correct) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_length_squared() {
        let a: Vector3 = Vector3::new(2.0, 0.0, 0.0);
        let a_correct: f32 = 4.0;
        assert!(f32::abs(a.length_squared() - a_correct) < F32_TEST_LIMIT);

        let b: Vector3 = Vector3::new(0.0, 3.0, 0.0);
        let b_correct: f32 = 9.0;
        assert!(f32::abs(b.length_squared() - b_correct) < F32_TEST_LIMIT);

        let c: Vector3 = Vector3::new(0.0, 0.0, 4.0);
        let c_correct: f32 = 16.0;
        assert!(f32::abs(c.length_squared() - c_correct) < F32_TEST_LIMIT);

        let d: Vector3 = Vector3::new(1.1, 1.2, 1.3);
        let d_correct: f32 = 4.34000000000014222336;
        assert!(f32::abs(d.length_squared() - d_correct) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_dot() {
        let correct: f32 = 12.0;

        let a: Vector3 = Vector3::new(1.0, 2.0, 3.0);
        let b: Vector3 = Vector3::new(4.0, -5.0, 6.0);
        let result: f32 = Vector3::dot(&a, &b) - correct;

        assert!(f32::abs(result) < F32_TEST_LIMIT);
    }
    
    #[test]
    fn test_vector3_cross() {
        let correct: Vector3 = Vector3::new(-1.0, -38.0, -16.0);

        let a: Vector3 = Vector3::new(4.0, 2.0, -5.0);
        let b: Vector3 = Vector3::new(2.0, -3.0, 7.0);
        let result: Vector3 = Vector3::cross(&a, &b) - correct;

        assert!(f32::abs(sum(result)) < F32_TEST_LIMIT);
    }
    
    #[test]
    fn test_vector3_cross_into() {
        let correct: Vector3 = Vector3::new(-1.0, -38.0, -16.0);

        let a: Vector3 = Vector3::new(4.0, 2.0, -5.0);
        let b: Vector3 = Vector3::new(2.0, -3.0, 7.0);

        let mut result: Vector3 = Vector3::zero();
        Vector3::cross_into(&a, &b, &mut result);
        let result: Vector3 = result - correct;

        assert!(f32::abs(sum(result)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_normalized() {
        let correct_scalar: f32 = 0.57735026918925152901829780358145;
        let correct: Vector3 = Vector3::new(correct_scalar, correct_scalar, correct_scalar);
        let a: Vector3 = Vector3::new(1.0, 1.0, 1.0);
        let result: Vector3 = a.get_normalized() - correct;

        assert!(f32::abs(sum(result)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_normalized_into() {
        let correct_scalar: f32 = 0.57735026918925152901829780358145;
        let correct: Vector3 = Vector3::new(correct_scalar, correct_scalar, correct_scalar);
        let a: Vector3 = Vector3::new(1.0, 1.0, 1.0);
        let mut result: Vector3 = Vector3::zero();
        a.get_normalized_into(&mut result);
        let result: Vector3 = result - correct;

        assert!(f32::abs(sum(result)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_normalize() {
        let correct_scalar: f32 = 0.57735026918925152901829780358145;
        let correct: Vector3 = Vector3::new(correct_scalar, correct_scalar, correct_scalar);
        let mut a: Vector3 = Vector3::new(1.0, 1.0, 1.0);
        a.normalize();
        let result: Vector3 = a - correct;

        assert!(f32::abs(sum(result)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_reflect() {
        let correct: Vector3 = Vector3::new(-2925.56, -5118.34, -465.76);

        let v: Vector3 = Vector3::new(1.0, 3.14, 22.0);
        let normal: Vector3 = Vector3::new(12.0, 21.0, 2.0);
        let mut reflected_out: Vector3 = Vector3::zero();

        Vector3::reflect(&v, &normal, &mut reflected_out);

        let result: Vector3 = reflected_out - correct;

        assert!(f32::abs(sum(result)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_refract() {
        let correct: Vector3 = Vector3::new(-115915.484, -202847.72, -19250.69);
        
        let v: Vector3 = Vector3::new(1.0, 3.14, 22.0);
        let n: Vector3 = Vector3::new(12.0, 21.0, 2.0);
        let etai_over_etat: f32 = 3.14;
        let mut refracted_out: Vector3 = Vector3::zero();

        Vector3::refract(&v, &n, etai_over_etat, &mut refracted_out);
        
        let result: Vector3 = refracted_out - correct;

        assert!(f32::abs(sum(result)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_index() {
        let a: Vector3 = Vector3::new(1.0 / 2.3, 2.5 / 4.2, 3.2 / 1.2);

        assert!(a.x == a[0] && a.y == a[1] && a.z == a[2]);
    }

    #[test]
    fn test_vector3_mul_add() {
        let a: Vector3 = Vector3::new(13.0 / 2.3, 2.5 / 4.2, 3.2 / 1.2);
        let b: Vector3 = Vector3::new(17.0 / 1.3, 2.5 / 4.2, 3.2 / 1.2);
        let c: Vector3 = Vector3::new(11.0 / 6.3, 24.5 / 4.2, 32.2 / 1.2);
        let d: Vector3 = Vector3::new(112.0 / 2.3, 245.5 / 4.2, 313.2 / 1.2);

        let result: Vector3 = Vector3::mul_add(&a, &b, &Vector3::mul_add(&c, &d, &b));
        let correct: Vector3 = Vector3::new(172.01411, 341.9218, 7013.278);

        let result: Vector3 = result - correct;

        assert!(f32::abs(result.x) < F32_TEST_LIMIT && f32::abs(result.y) < F32_TEST_LIMIT && f32::abs(result.z) < F32_TEST_LIMIT);
        assert!(f32::abs(sum(result)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_scalar_mul_add() {
        let a: Vector3 = Vector3::new(13.0 / 2.3, 2.5 / 4.2, 3.2 / 1.2);
        let b: f32 = 3.14;
        let c: Vector3 = Vector3::new(11.0 / 6.3, 24.5 / 4.2, 32.2 / 1.2);
        let d: f32 = -2.145;

        let result: Vector3 = Vector3::scalar_mul_add(&a, b, &Vector3::scalar_mul_add(&c, d, &a));
        let correct: Vector3 = Vector3::new(19.654762, -10.048214, -46.517498);

        let result: Vector3 = result - correct;

        assert!(f32::abs(result.x) < F32_TEST_LIMIT && f32::abs(result.y) < F32_TEST_LIMIT && f32::abs(result.z) < F32_TEST_LIMIT);
        assert!(f32::abs(sum(result)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_mul_scalar_add() {
        let a: Vector3 = Vector3::new(-13.0 / 2.3, 2.5 / 4.2, -3.2 / 1.2);
        let b: f32 = 3.14;
        let c: Vector3 = Vector3::new(11.0 / 6.3, -24.5 / 4.2, 32.2 / 1.2);
        let d: f32 = -2.145;

        let result: Vector3 = Vector3::mul_scalar_add(&a, &Vector3::mul_scalar_add(&c, &a, d), b);
        let correct: Vector3 = Vector3::new(71.04451, -0.2035852, 199.67477);

        let result: Vector3 = result - correct;

        assert!(f32::abs(result.x) < F32_TEST_LIMIT && f32::abs(result.y) < F32_TEST_LIMIT && f32::abs(result.z) < F32_TEST_LIMIT);
        assert!(f32::abs(sum(result)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_vector3_scalar_mul_scalar_add() {
        let a: Vector3 = Vector3::new(-13.0 / 2.3, 2.5 / 4.2, -3.2 / 1.2);
        let b: f32 = 3.14;
        let c: f32 = -2.145;

        let result: Vector3 = Vector3::scalar_mul_scalar_add(&a, b, c);
        let correct: Vector3 = Vector3::new(-19.892826, -0.27595213, -10.518333);

        let result: Vector3 = result - correct;

        assert!(f32::abs(result.x) < F32_TEST_LIMIT && f32::abs(result.y) < F32_TEST_LIMIT && f32::abs(result.z) < F32_TEST_LIMIT);
        assert!(f32::abs(sum(result)) < F32_TEST_LIMIT);
    }

}