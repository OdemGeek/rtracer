use nalgebra::Vector3;

#[derive(Debug, Default)]
pub struct Material {
    pub albedo: Vector3<f32>,
    pub emission: Vector3<f32>,
    pub roughness: f32,
    pub metallic: f32,
}

impl Material {
    #[inline]
    pub fn new(albedo: Vector3<f32>, emission: Vector3<f32>, roughness: f32, metallic: f32) -> Self {
        Material { albedo, emission, roughness, metallic }
    }
}
