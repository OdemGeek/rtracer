use nalgebra::{Vector3, Vector2};
use crate::{scene::SceneData, math::ray::Ray};
//use crate::math::extensions::{direction_to_euler, euler_to_direction, f32_vector3_from_u32};
//use std::f32::consts::{PI, FRAC_PI_2};

pub trait Shader {
    fn frag(screen_pos: &Vector2<f32>, position: &Vector3<f32>, normal: &Vector3<f32>, scene: &SceneData) -> Vector3<f32>;
}

pub struct TestShader{}

#[allow(unused_variables)]
impl Shader for TestShader {
    fn frag(screen_pos: &Vector2<f32>, position: &Vector3<f32>, normal: &Vector3<f32>, scene: &SceneData) -> Vector3<f32> {
        let sun_direction = Vector3::<f32>::new(0.5, 0.5, -0.5);
        let sun_ray = Ray::new(position.clone() + sun_direction * 0.01, sun_direction.clone());
        let sun_atten = if scene.cast_ray(&sun_ray).is_some() {0.0f32} else {1.0f32};
        let light = normal.dot(&sun_direction).clamp(0.0, 1.0) * sun_atten;
        
        let norm = (*normal * 0.5 + Vector3::new(0.5, 0.5, 0.5)) * light;

        Vector3::<f32>::new(norm.x, norm.y, norm.z)
    }
}

pub struct SkyShader{}

#[allow(unused_variables)]
impl Shader for SkyShader {
    fn frag(screen_pos: &Vector2<f32>, position: &Vector3<f32>, normal: &Vector3<f32>, scene: &SceneData) -> Vector3<f32> {
        // Convertion test code
        /*let e = direction_to_euler(normal);
        let n = euler_to_direction(&e);
        let norm = n * 0.5 + Vector3::new(0.5, 0.5, 0.5);

        Vector3::<f32>::new(norm.x, norm.y, norm.z)*/
        /*let euler_angles = direction_to_euler(&normal);
        let (u, v) = uv_on_sphere(&euler_angles);
        let x = u * texture_size.0 as f32;
        //let x = screen_pos.x;
        let y = v * texture_size.1 as f32;
        //let y = screen_pos.y;
        f32_vector3_from_u32(texture[(y * texture_size.0 as f32 + x) as usize]) / 255.0*/

        let norm = normal * 0.5 + Vector3::new(0.5, 0.5, 0.5);
        Vector3::<f32>::new(norm.x, norm.y, norm.z)
    }
}

#[allow(dead_code)]
fn uv_on_sphere(euler: &Vector3<f32>) -> (f32, f32) {
    let u = 0.5 + f32::atan2(euler.z.to_radians(), euler.x.to_radians()) / (2.0 * std::f32::consts::PI);
    let v = 0.5 + euler.y.to_radians().asin() / std::f32::consts::PI;
    (u, v)
}
