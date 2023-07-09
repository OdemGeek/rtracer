use crate::scene::SceneData;
use crate::camera::Camera;
use crate::shaders::{Shader, TestShader, SkyShader};
use crate::math::extensions::*;
use nalgebra::Vector2;
use rayon::prelude::*;

pub struct Render {
    pub texture_buffer: Vec<u32>,
    old_texture_buffer: Vec<u32>,
    seed: u32,
}

impl Render {
    pub fn new(tex_buffer: &Vec<u32>) -> Self {
        Render { texture_buffer: tex_buffer.clone(), old_texture_buffer: tex_buffer.clone(), seed: 0 }
    }

    pub fn draw(&mut self, scene: &SceneData, camera: &Camera, frame_index: u32) {
        // Iterate over the pixels of the image
        self.texture_buffer.par_iter_mut().enumerate().for_each(|(i, pixel)| {
            let x = i % camera.screen_width as usize;
            let y = camera.screen_height as usize - i / camera.screen_width as usize;
            let mut seed = frame_index;
            let screen_pos = Vector2::<f32>::new(x as f32 / camera.screen_width as f32, y as f32 / camera.screen_height as f32);
            // Get camera ray
            let ray = camera.ray_from_screen_point(Vector2::<f32>::new(x as f32, y as f32), &mut seed);
            // Calculate intersection
            let hit = scene.cast_ray(&ray);

            // Calculate fragment
            let mut color;
            if let Some(hit_value) = hit {
                let point = ray.origin + ray.direction * hit_value.t;
                let normal = (point - hit_value.object.anchor.position).normalize();
                color = TestShader::frag(&screen_pos, &point, &normal, &scene);
            } else {
                color = SkyShader::frag(&screen_pos, &ray.direction, &ray.direction, &scene);
            }
            
            let weight = 1.0 / (frame_index as f32 + 1.0);
            let old_color = f32_vector3_from_u32(self.old_texture_buffer[i]);
            // Clamp color in range 0-1
            color.x = color.x.clamp(0.0, 1.0);
            color.y = color.y.clamp(0.0, 1.0);
            color.z = color.z.clamp(0.0, 1.0);
            let blended_color = old_color * (1.0 - weight) + color * weight; // TODO: Fix blending, now it's works only with dark colors
            
            // Convert 0-1 range to 0-255
            let final_color = blended_color * 255.0;
            *pixel = u32_from_u8_rgb(final_color.x as u8, final_color.y as u8, final_color.z as u8);
        });
        println!("{}", frame_index);
        self.old_texture_buffer = self.texture_buffer.clone();
    }
}