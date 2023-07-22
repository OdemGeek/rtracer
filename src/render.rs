use crate::entity::hittable::Hittable;
use crate::math::pcg::{self, random_direction};
use crate::scene::SceneData;
use crate::camera::Camera;
//use crate::shaders::{Shader, TestShader, SkyShader};
use crate::math::extensions::*;
use nalgebra::{Vector2, Vector3};
use rayon::prelude::*;

pub struct Render {
    pub texture_buffer: Vec<Vector3<f32>>,
    accumulated_frames: u32,
    seed: u32,
}

impl Render {
    #[inline]
    pub fn new(width: u32, height: u32) -> Self {
        Render { texture_buffer: vec![Vector3::zeros(); (width * height) as usize], accumulated_frames: 0, seed: 153544 }
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

            //let screen_pos = Vector2::<f32>::new(x as f32 / camera.screen_width as f32, y as f32 / camera.screen_height as f32);
            // Get camera ray
            let mut ray = camera.ray_from_screen_point(Vector2::<f32>::new(x as f32, y as f32), &mut seed);
            
            let mut color = Vector3::new(1.0, 1.0, 1.0);
            let mut light = Vector3::zeros();

            const MAX_BOUNCES: u32 = 16;
            for _ in 0..MAX_BOUNCES {
                // Calculate intersection
                let hit = scene.cast_ray(&ray);
                // Calculate fragment
                if let Some(hit_value) = hit {
                    let material = &hit_value.object.material;
                    let point = ray.origin + ray.direction * hit_value.t;
                    let normal = hit_value.object.normal(&point);

                    ray.origin = point + normal * 0.001;
                    let reflection = reflect(&ray.direction, &normal);
                    let diffuse = (normal + random_direction(&mut seed)).normalize();
                    ray.direction = lerp_vector3(&reflection, &diffuse, material.roughness).normalize();
                    
                    light += material.emission.component_mul(&color);
                    color = color.component_mul(&material.albedo) * 0.25;
                } else {
                    light += Vector3::new(0.2, 0.2, 0.2).component_mul(&color);
                    break;
                }
            }

            // Blend generated pixel with old one
            let blended_color = lerp_vector3(pixel, &light, weight);
            *pixel = blended_color;
        });
        self.accumulated_frames += 1;
    }

    #[inline]
    pub fn reset_accumulated_frames(&mut self) {
        self.accumulated_frames = 0;
    }

    #[inline]
    pub fn get_accumulated_frames_count(&self) -> u32 {
        self.accumulated_frames
    }
}
