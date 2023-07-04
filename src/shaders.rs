use nalgebra::{Vector3, Vector2};
use crate::math::extensions::direction_to_euler;
use crate::math::extensions::euler_to_direction;
use std::f32::consts::{PI, FRAC_PI_2};

pub trait Shader {
    fn frag(screen_pos: &Vector2<f32>, normal: &Vector3<f32>, texture: &Vec<u32>, texture_size: &(u32, u32)) -> Vector3<f32>;
}

pub struct TestShader{}

#[allow(unused_variables)]
impl Shader for TestShader {
    fn frag(screen_pos: &Vector2<f32>, normal: &Vector3<f32>, texture: &Vec<u32>, texture_size: &(u32, u32)) -> Vector3<f32> {
        let light = normal.dot(&Vector3::<f32>::new(0.5, 0.5, -0.5)).clamp(0.0, 1.0);
        
        let norm = (*normal * 0.5 + Vector3::new(0.5, 0.5, 0.5)) * light;

        Vector3::<f32>::new(norm.x, norm.y, norm.z)
    }
}

pub struct SkyShader{}

#[allow(unused_variables)]
impl Shader for SkyShader {
    fn frag(screen_pos: &Vector2<f32>, normal: &Vector3<f32>, texture: &Vec<u32>, texture_size: &(u32, u32)) -> Vector3<f32> {
        // Convertion test code
        /*let e = direction_to_euler(normal);
        let n = euler_to_direction(&e);
        let norm = n * 0.5 + Vector3::new(0.5, 0.5, 0.5);

        Vector3::<f32>::new(norm.x, norm.y, norm.z)*/
        let euler_angles = direction_to_euler(&normal);
        let (u, v) = uv_on_sphere(&euler_angles);
        let x = u * texture_size.0 as f32;
        //let x = screen_pos.x;
        let y = v * texture_size.1 as f32;
        //let y = screen_pos.y;
        crate::f32_vector3_from_u32(texture[(y * texture_size.0 as f32 + x) as usize]) / 255.0
    }
}

fn uv_on_sphere(euler: &Vector3<f32>) -> (f32, f32) {
    let u = 0.5 + f32::atan2(euler.z.to_radians(), euler.x.to_radians()) / (2.0 * std::f32::consts::PI);
    let v = 0.5 + euler.y.to_radians().asin() / std::f32::consts::PI;
    (u, v)
}
