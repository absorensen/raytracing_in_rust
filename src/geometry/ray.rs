use crate::geometry::vector3::Vector3;

#[derive(Copy, Clone, Debug)]
struct Ray {
    pub origin : Vector3,
    pub direction: Vector3,
    pub time: f64,
}

impl Ray {
    pub fn point_at_parameter(&self, t:f64) -> Vector3 {
        self.origin + t * self.direction
    }
}