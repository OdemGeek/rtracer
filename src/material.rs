use nalgebra::Vector3;

use crate::textures::texture::Texture;

#[derive(Debug, Default)]
pub struct Material {
    pub albedo: Vector3<f32>,
    pub emission: Vector3<f32>,
    pub roughness: f32,
    pub metallic: f32,
    pub albedo_tex: Option<Texture<u32>>
}

impl Material {
    #[inline]
    pub fn new(albedo: Vector3<f32>, emission: Vector3<f32>, roughness: f32, metallic: f32,
        albedo_tex: Option<Texture<u32>>) -> Self {
        Material { albedo, emission, roughness, metallic, albedo_tex }
    }
}
