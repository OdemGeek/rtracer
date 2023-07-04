use nalgebra::{Vector3, Vector2};

pub trait Shader {
    fn frag(screen_pos: &Vector2<f32>, normal: &Vector3<f32>) -> Vector3<f32>;
}

pub struct TestShader{}

#[allow(unused_variables)]
impl Shader for TestShader {
    fn frag(screen_pos: &Vector2<f32>, normal: &Vector3<f32>) -> Vector3<f32> {
        let light = normal.dot(&Vector3::<f32>::new(0.5, 0.5, -0.5));
        let norm = (*normal * 0.5 + Vector3::new(0.5, 0.5, 0.5)) * light;
        
        Vector3::<f32>::new(norm.x, norm.y, norm.z)
    }
}

pub struct SkyShader{}

#[allow(unused_variables)]
impl Shader for SkyShader {
    fn frag(screen_pos: &Vector2<f32>, normal: &Vector3<f32>) -> Vector3<f32> {
        *normal * 0.5 + Vector3::new(0.5, 0.5, 0.5)
    }
}