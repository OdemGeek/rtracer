use crate::math::vectors::{Float3, Float2, Vector};

pub trait Shader {
    fn frag(screen_pos: &Float2, normal: &Float3) -> Float3;
}

pub struct TestShader{}

#[allow(unused_variables)]
impl Shader for TestShader {
    fn frag(screen_pos: &Float2, normal: &Float3) -> Float3 {
        let light = Float3::dot(&Float3::new(0.5, 0.5, 0.5), &normal);
        let norm = (*normal * 0.5 + Float3::one() * 0.5) * light;
        
        Float3::new(norm.x, norm.y, norm.z)
    }
}

pub struct SkyShader{}

#[allow(unused_variables)]
impl Shader for SkyShader {
    fn frag(screen_pos: &Float2, normal: &Float3) -> Float3 {
        Float3::new(screen_pos.x, screen_pos.y, 0.0)
    }
}