use nalgebra::{Vector3, Vector2};
use crate::scene::SceneData;

pub trait Shader {
    fn frag(screen_pos: &Vector2<f32>, position: &Vector3<f32>, normal: &Vector3<f32>, scene: &SceneData) -> Vector3<f32>;
}

pub struct TestShader{}

#[allow(unused_variables)]
impl Shader for TestShader {
    #[inline]
    fn frag(screen_pos: &Vector2<f32>, position: &Vector3<f32>, normal: &Vector3<f32>, scene: &SceneData) -> Vector3<f32> {
        let norm = *normal * 0.5 + Vector3::new(0.5, 0.5, 0.5);
        Vector3::<f32>::new(norm.x, norm.y, norm.z)
    }
}

pub struct SkyShader{}

#[allow(unused_variables)]
impl Shader for SkyShader {
    #[inline]
    fn frag(screen_pos: &Vector2<f32>, position: &Vector3<f32>, normal: &Vector3<f32>, scene: &SceneData) -> Vector3<f32> {
        let norm = normal * 0.5 + Vector3::new(0.5, 0.5, 0.5);
        Vector3::<f32>::new(norm.x, norm.y, norm.z)
    }
}

#[allow(dead_code)]
#[inline]
fn uv_on_sphere(euler: &Vector3<f32>) -> (f32, f32) {
    let u = 0.5 + f32::atan2(euler.z.to_radians(), euler.x.to_radians()) / (2.0 * std::f32::consts::PI);
    let v = 0.5 + euler.y.to_radians().asin() / std::f32::consts::PI;
    (u, v)
}
