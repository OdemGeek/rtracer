use crate::entity::bvh::Bvh;
use crate::entity::hit::Intersection;
use crate::math::pcg::{self, random_direction, random_vector3};
//use crate::math::ray::Ray;
use crate::scene::SceneData;
use crate::camera::Camera;
use crate::math::extensions::*;
use nalgebra::{Vector2, Vector3};
use rayon::prelude::*;

pub struct Render {
    pub texture_buffer: Vec<Vector3<f32>>,
    pub debug_depth: u32,
    pub bvh_debug: bool,
    accumulated_frames: u32,
    seed: u32,
}

impl Render {
    #[inline]
    pub fn new(width: u32, height: u32) -> Self {
        Render { texture_buffer: vec![Vector3::zeros(); (width * height) as usize], accumulated_frames: 0, seed: 153544, debug_depth: 0, bvh_debug: false }
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
            let mut ray = camera.ray_from_screen_point(&Vector2::<f32>::new(x as f32, y as f32), &mut seed);
            
            let mut color: Vector3<f32> = Vector3::new(1.0, 1.0, 1.0);
            let mut light: Vector3<f32> = Vector3::zeros();

            if self.bvh_debug {
                let bvhs: Vec<&Bvh> = scene.get_bvh_by_depth(self.debug_depth);
                color = Vector3::new(0.005, 0.005, 0.005);
                let mut hits: Vec<Intersection<Bvh>> = bvhs.iter().filter_map(|x| x.intersect_point(&ray)).flatten().collect();
                hits.sort_by(|x, y| y.t.partial_cmp(&x.t).unwrap());
                
                for hit in hits {
                    let mut bvh_color: Vector3<f32> = random_vector3(&mut ((hit.object.first_object + hit.object.object_count) as u32));
                    let hit_point = ray.origin + ray.direction * hit.t;
                    let distance_to_edge = (hit.object.distance_to_edge(&hit_point) * 15.0).clamp(0.0, 1.0);
                    let edge_color = Vector3::new(0.9, 0.9, 0.9);
                    bvh_color = lerp_vector3(&edge_color, &bvh_color, distance_to_edge.clamp(0.0, 1.0));
                    color = lerp_vector3(&color, &bvh_color, 0.3);
                }

                let blended_color = lerp_vector3(pixel, &color, weight);
                *pixel = blended_color;
                return;
            }
            const MAX_BOUNCES: u32 = 3;
            for _ in 0..MAX_BOUNCES {
                // Calculate intersection
                seed = pcg::hash(seed);
                let hit_option = scene.cast_ray(&ray);
                // Calculate fragment
                if let Some(hit) = hit_option {
                    let material = &hit.object.material;

                    ray.origin = hit.point + hit.normal * 0.001;
                    let reflection: Vector3<f32> = reflect(&ray.direction, &hit.normal);
                    let diffuse: Vector3<f32> = (hit.normal + random_direction(&mut seed)).normalize();

                    ray.direction = lerp_vector3(&reflection, &diffuse, material.roughness).normalize();
                    
                    light += material.emission.component_mul(&color);
                    color = color.component_mul(&material.albedo);

                } else {
                    light += Vector3::new(0.3, 0.3, 0.3).component_mul(&color);
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
        self.seed = 153544;
    }

    #[inline]
    pub fn get_accumulated_frames_count(&self) -> u32 {
        self.accumulated_frames
    }
}
