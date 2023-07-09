use crate::scene::SceneData;
use crate::camera::Camera;
use crate::shaders::{Shader, TestShader, SkyShader};
use crate::math::extensions::u32_from_u8_rgb;
use nalgebra::Vector2;
use rayon::prelude::*;

pub struct Render {
    pub texture_buffer: Vec<u32>
}

impl Render {
    pub fn new(tex_buffer: Vec<u32>) -> Self {
        Render { texture_buffer: tex_buffer }
    }

    pub fn draw(&mut self, scene: &SceneData, camera: &Camera) {
        // Iterate over the pixels of the image
        self.texture_buffer.par_iter_mut().enumerate().for_each(|(i, pixel)| {
            let x = i % camera.screen_width as usize;
            let y = camera.screen_height as usize - i / camera.screen_width as usize;
            let screen_pos = Vector2::<f32>::new(x as f32 / camera.screen_width as f32, y as f32 / camera.screen_height as f32);
            // Get camera ray
            let ray = camera.ray_from_screen_point(Vector2::<f32>::new(x as f32, y as f32));
            // Calculate intersection
            let hit = scene.cast_ray(&ray);

            // Calculate fragment
            let color;
            if let Some(hit_value) = hit {
                let point = ray.origin + ray.direction * hit_value.t;
                let normal = (point - hit_value.object.anchor.position).normalize();
                color = TestShader::frag(&screen_pos, &normal/*, &skybox, &skybox_dimensions*/);
            } else {
                color = SkyShader::frag(&screen_pos, &ray.direction/*, &skybox, &skybox_dimensions*/);
            }
            
            // Convert Float3 to Rgb
            let final_color = color * 255.0;
            *pixel = u32_from_u8_rgb(final_color.x as u8, final_color.y as u8, final_color.z as u8);
        });
    }
}