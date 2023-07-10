use nalgebra::Vector3;

pub struct Material {
    pub albedo: Vector3<f32>,
    pub emission: Vector3<f32>,
}

impl Material {
    pub fn new(albedo: Vector3<f32>, emission: Vector3<f32>) -> Self {
        Material { albedo: albedo, emission: emission }
    }
}