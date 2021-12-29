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
        (*v) - 2.0 * Vector3::dot(v, normal) * (*normal)
    }

    pub fn refract(v: &Vector3, n: &Vector3, etai_over_etat: f64) -> Option<Vector3> {
        let uv = v.normalized();
        let dt = Vector3::dot(&uv,&n);
        let discriminant = 1.0 - etai_over_etat.powi(2) * (1.0 - dt.powi(2));
        if discriminant > 0.0 {
            let refracted = etai_over_etat * (uv - *n * dt) - *n * discriminant.sqrt();
            Some(refracted)
        } else {
            None
        }
    }

}