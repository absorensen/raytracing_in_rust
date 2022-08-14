
#[cfg(test)]
mod tests {
    use crate::core::color_rgb::{ColorRGB};

    const F32_TEST_LIMIT: f32 = f32::EPSILON;

    #[inline]
    fn sum(color: &ColorRGB) -> f32 {
        color.r + color.g + color.b
    }

    #[inline]
    fn approximately_equal(color:&ColorRGB, reference: &ColorRGB) -> bool {
        let difference: ColorRGB = *color - *reference;

        f32::abs(difference.r) < F32_TEST_LIMIT && f32::abs(difference.g) < F32_TEST_LIMIT && f32::abs(difference.b) < F32_TEST_LIMIT
    }

    #[test]
    fn test_color_rgb_black() {
        let a: ColorRGB = ColorRGB::black();

        assert!(a.r < F32_TEST_LIMIT && a.g < F32_TEST_LIMIT && a.b < F32_TEST_LIMIT);
    }

    #[test]
    fn test_color_rgb_white() {
        let a: ColorRGB = ColorRGB::white();

        assert!((a.r - 1.0) < F32_TEST_LIMIT && (a.g - 1.0) < F32_TEST_LIMIT && (a.b - 1.0) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_color_rgb_new() {
        let correct_scalar: f32 = 0.6;
        let correct_struct: ColorRGB = ColorRGB{r:0.1, g: 0.2, b: 0.3};
        
        let a: ColorRGB = ColorRGB::new(0.1, 0.2, 0.3);

        let result: f32 = sum(&a) - correct_scalar;

        assert!(f32::abs(result) < F32_TEST_LIMIT);
        assert!(approximately_equal(&a, &correct_struct))
    }

    #[test]
    fn test_color_rgb_random() {
        let upper_limit: f32 = 1.0;
        let lower_limit: f32 = 0.0;

        let iteration_count: usize = 1000;
        
        let mut rng = rand::thread_rng();

        for _iteration_index in 0..iteration_count {
            let a: ColorRGB = ColorRGB::random(&mut rng);

            assert!(
                lower_limit <= a.r && 
                a.r < upper_limit && 
                lower_limit <= a.g &&
                a.g < upper_limit &&
                lower_limit <= a.b &&
                a.b < upper_limit
            );
        }
    }

    #[test]
    fn test_color_rgb_add() {
        let correct: ColorRGB = ColorRGB::new(3.3, 6.7, 4.4);

        let a: ColorRGB = ColorRGB::new(1.0, 2.5, 3.2);
        let b: ColorRGB = ColorRGB::new(2.3, 4.2, 1.2);
        let result: ColorRGB = a + b - correct;

        assert!(f32::abs(sum(&result)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_color_rgb_div() {
        let correct: ColorRGB = ColorRGB::new(1.0 / 2.3, 2.5 / 4.2, 3.2 / 1.2);

        let a: ColorRGB = ColorRGB::new(1.0, 2.5, 3.2);
        let b: ColorRGB = ColorRGB::new(2.3, 4.2, 1.2);
        let result: ColorRGB = a / b - correct;

        assert!(f32::abs(sum(&result)) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_color_rgb_index() {
        let a: ColorRGB = ColorRGB::new(1.0 / 2.3, 2.5 / 4.2, 3.2 / 1.2);

        assert!(a.r == a[0] && a.g == a[1] && a.b == a[2]);
    }

}