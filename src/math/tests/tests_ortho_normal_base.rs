#[cfg(test)]
mod tests {
    use crate::math::{vector3::Vector3, ortho_normal_base::OrthoNormalBase};

    const F32_TEST_LIMIT: f32 = f32::EPSILON;

    #[test]
    fn test_ortho_normal_base_build_from_w() {
        let n : Vector3 = Vector3::new(-5000.0, 3.14, -1623.0);

        let onb : OrthoNormalBase = OrthoNormalBase::build_from_w(&n);

        assert!(onb.u.x - (-0.00056813797) < F32_TEST_LIMIT);
        assert!(onb.u.y - (-0.9999999) < F32_TEST_LIMIT);
        assert!(onb.u.z - (-0.0001844176) < F32_TEST_LIMIT);

        assert!(onb.v.x - 0.30874196 < F32_TEST_LIMIT);
        assert!(onb.v.y - 0.0 < F32_TEST_LIMIT);
        assert!(onb.v.z - -0.9511459 < F32_TEST_LIMIT);

        assert!(onb.w.x - (-0.95114565) < F32_TEST_LIMIT);
        assert!(onb.w.y - 0.0005973195 < F32_TEST_LIMIT);
        assert!(onb.w.z - (-0.3087419) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_ortho_normal_base_update() {
        let m : Vector3 = Vector3::new(-1.0, 3.14, -3.0);
        let n : Vector3 = Vector3::new(-5000.0, 3.14, -1623.0);


        let mut onb : OrthoNormalBase = OrthoNormalBase::build_from_w(&n);
        onb.update(m);


        assert!(onb.u.x - (-0.97449815) < F32_TEST_LIMIT);
        assert!(onb.u.y - (-0.16224755) < F32_TEST_LIMIT);
        assert!(onb.u.z - 0.1550136 < F32_TEST_LIMIT);

        assert!(onb.v.x - 0.0 < F32_TEST_LIMIT);
        assert!(onb.v.y - (-0.6908043) < F32_TEST_LIMIT);
        assert!(onb.v.z - (-0.7230418) < F32_TEST_LIMIT);

        assert!(onb.w.x - (-0.22439583) < F32_TEST_LIMIT);
        assert!(onb.w.y - 0.7046029 < F32_TEST_LIMIT);
        assert!(onb.w.z - (-0.6731875) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_ortho_normal_base_local_vector() {
        let m : Vector3 = Vector3::new(-1.0, 3.14, -3.0);
        let n : Vector3 = Vector3::new(-5000.0, 3.14, -1623.0);
        let a : Vector3 = Vector3::new(-3214.0, -1.2, 23.1);


        let mut onb : OrthoNormalBase = OrthoNormalBase::build_from_w(&n);
        onb.update(m);

        let local_vector : Vector3 = onb.local_vector(&a);

        assert!(local_vector.x - 3126.8535 < F32_TEST_LIMIT);
        assert!(local_vector.y - 538.5689 < F32_TEST_LIMIT);
        assert!(local_vector.z - (-512.8967) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_ortho_normal_base_index() {
        let m : Vector3 = Vector3::new(-1.0, 3.14, -3.0);
        let n : Vector3 = Vector3::new(-5000.0, 3.14, -1623.0);

        let mut onb : OrthoNormalBase = OrthoNormalBase::build_from_w(&n);
        onb.update(m);


        let u : Vector3 = onb[0];
        let v : Vector3 = onb[1];
        let w : Vector3 = onb[2];

        assert!(u.x - (-0.97449815) < F32_TEST_LIMIT);
        assert!(u.y - (-0.16224755) < F32_TEST_LIMIT);
        assert!(u.z - 0.1550136 < F32_TEST_LIMIT);

        assert!(v.x - 0.0 < F32_TEST_LIMIT);
        assert!(v.y - (-0.6908043) < F32_TEST_LIMIT);
        assert!(v.z - (-0.7230418) < F32_TEST_LIMIT);

        assert!(w.x - (-0.22439583) < F32_TEST_LIMIT);
        assert!(w.y - 0.7046029 < F32_TEST_LIMIT);
        assert!(w.z - (-0.6731875) < F32_TEST_LIMIT);
    }

    #[test]
    fn test_ortho_normal_base_index_mut() {
        let m : Vector3 = Vector3::new(-1.0, 3.14, -3.0);
        let n : Vector3 = Vector3::new(-5000.0, 3.14, -1623.0);

        let mut onb : OrthoNormalBase = OrthoNormalBase::build_from_w(&n);
        onb.update(m);


        let mut u : Vector3 = onb[0];
        u.x -= 0.1;

        let mut v : Vector3 = onb[1];
        v.y -= 0.2;
        
        let mut w : Vector3 = onb[2];
        w.z += 0.3;

        assert!(u.x - (-1.0744982) < F32_TEST_LIMIT);
        assert!(u.y - (-0.16224755) < F32_TEST_LIMIT);
        assert!(u.z - 0.1550136 < F32_TEST_LIMIT);

        assert!(v.x - 0.0 < F32_TEST_LIMIT);
        assert!(v.y - (-0.8908043) < F32_TEST_LIMIT);
        assert!(v.z - (-0.7230418) < F32_TEST_LIMIT);

        assert!(w.x - (-0.22439583) < F32_TEST_LIMIT);
        assert!(w.y - 0.7046029 < F32_TEST_LIMIT);
        assert!(w.z - (-0.37318748) < F32_TEST_LIMIT);
    }

}