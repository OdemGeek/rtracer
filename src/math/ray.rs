use nalgebra::Vector3;

#[allow(dead_code)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

#[allow(dead_code)]
impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Self {
        Ray { origin, direction }
    }
}