use crate::pcg;
use crate::scene::SceneData;
use crate::camera::Camera;
use crate::shaders::{Shader, TestShader, SkyShader};
use crate::math::extensions::*;
use nalgebra::{Vector2, Vector3};
use rayon::prelude::*;

pub struct Render {
    pub texture_buffer: Vec<Vector3<f32>>,
    //old_texture_buffer: Vec<u32>,
    accumulated_frames: u32,
    //combined_frames: u32,
    seed: u32,
}

impl Render {
    #[inline]
    pub fn new(width: u32, height: u32) -> Self {
        Render { texture_buffer: vec![Vector3::zeros(); (width * height) as usize], accumulated_frames: 0, seed: 153544 }
        //Render { texture_buffer: tex_buffer.clone(), old_texture_buffer: tex_buffer.clone(), accumulated_frames: 0, combined_frames: 0 }
    }

    pub fn draw(&mut self, scene: &SceneData, camera: &Camera) {
        // Weight of current frame
        let weight = 1.0 / (self.accumulated_frames + 1) as f32;
        self.seed = pcg::hash(self.seed);
        // Iterate over the pixels of the image
        self.texture_buffer.par_iter_mut().enumerate().for_each(|(i, pixel)| {
            let x = i % camera.screen_width as usize;
            let y = camera.screen_height as usize - i / camera.screen_width as usize;
            let mut seed = self.seed.wrapping_mul(x as u32).wrapping_mul(y as u32);

            let screen_pos = Vector2::<f32>::new(x as f32 / camera.screen_width as f32, y as f32 / camera.screen_height as f32);
            // Get camera ray
            let ray = camera.ray_from_screen_point(Vector2::<f32>::new(x as f32, y as f32), &mut seed);
            // Calculate intersection
            let hit = scene.cast_ray(&ray);

            // Calculate fragment
            let color;
            if let Some(hit_value) = hit {
                let point = ray.origin + ray.direction * hit_value.t;
                let normal = (point - hit_value.object.anchor.position).normalize();
                color = TestShader::frag(&screen_pos, &point, &normal, &scene);
            } else {
                color = SkyShader::frag(&screen_pos, &ray.direction, &ray.direction, &scene);
            }
            
            // Blend generated pixel with old one
            let blended_color = *pixel * (1.0 - weight) + color * weight;
            *pixel = blended_color;
        });
        self.accumulated_frames += 1;
    }

    #[inline]
    pub fn reset_accumulated_frames(&mut self) {
        self.accumulated_frames = 0;
    }
}
