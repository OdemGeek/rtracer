use crate::math::pcg::{self, random_direction, random_vector3, random_f32};
use crate::math::ray::Ray;
use crate::scene::SceneData;
use crate::camera::Camera;
use crate::math::extensions::*;
use crate::textures::texture::Texture;
use nalgebra::{Vector2, Vector3};
use rayon::prelude::*;

pub struct Render {
    pub texture_buffer: Vec<Vector3<f32>>,
    pub bvh_debug: bool,
    pub texture: Option<Texture<Vector3<f32>>>,
    accumulated_frames: u32,
    seed: u32
}

impl Render {
    #[inline]
    pub fn new(width: u32, height: u32, sky_texture: Option<Texture<Vector3<f32>>>) -> Self {
        Render {
            texture_buffer: vec![Vector3::zeros(); (width * height) as usize],
            bvh_debug: false,
            texture: sky_texture,
            accumulated_frames: 0,
            seed: 153544,
        }
    }

    pub fn draw(&mut self, scene: &SceneData, camera: &Camera) {
        // Weight of current frame
        let weight = 1.0 / (self.accumulated_frames + 1) as f32;
        self.seed = pcg::hash(self.seed);
        
        if self.bvh_debug {
            // Iterate over the pixels of the image
            self.texture_buffer.par_iter_mut().enumerate().for_each(|(i, pixel)| {
                let x = i % camera.screen_width as usize;
                let y = camera.screen_height as usize - i / camera.screen_width as usize;
                let mut seed = self.seed.wrapping_mul(x as u32).wrapping_mul(y as u32);
                
                // Get camera ray
                let mut ray: Ray = camera.ray_from_screen_point(&Vector2::new(x as f32, y as f32), &mut seed);
                let mut color: Vector3<f32> = Vector3::new(0.005, 0.005, 0.005);

                const TRANSMISSION_BOUNCES: u32 = 16;
                for _ in 0..TRANSMISSION_BOUNCES {
                    let scene_hit = scene.cast_debug_ray(&ray);
                    if let Some(hit) = scene_hit {
                        let hit_point = ray.origin + ray.get_direction() * hit.t;

                        let mut bvh_color: Vector3<f32> = random_vector3(&mut ((hit.object.first_object + hit.object.object_count) as u32));
                        let distance_to_edge = (hit.object.distance_to_edge(&hit_point) * 15.0).clamp(0.0, 1.0);
                        let edge_color = Vector3::new(0.9, 0.9, 0.9);
                        bvh_color = lerp_vector3(&edge_color, &bvh_color, distance_to_edge.clamp(0.0, 1.0));

                        ray.origin = hit.point + ray.get_direction() * 0.001;
                        color = lerp_vector3(&color, &bvh_color, 0.25);
                    } else {
                        break;
                    }
                }

                let blended_color = lerp_vector3(pixel, &color, weight);
                *pixel = blended_color;
            });
        } else {
            // Iterate over the pixels of the image
            self.texture_buffer.par_iter_mut().enumerate().for_each(|(i, pixel)| {
                let x = i % camera.screen_width as usize;
                let y = camera.screen_height as usize - i / camera.screen_width as usize;
                let mut seed = self.seed.wrapping_add(y as u32).wrapping_mul(x as u32).wrapping_add(x as u32).wrapping_mul(y as u32);

                //let screen_pos = Vector2::<f32>::new(x as f32 / camera.screen_width as f32, y as f32 / camera.screen_height as f32);
                // Get camera ray
                let mut ray: crate::math::ray::Ray = camera.ray_from_screen_point(&Vector2::new(x as f32, y as f32), &mut seed);
                let mut color: Vector3<f32> = Vector3::new(1.0, 1.0, 1.0);
                let mut light: Vector3<f32> = Vector3::zeros();

                const MAX_BOUNCES: u32 = 3;
                for bounce_index in 0..MAX_BOUNCES {
                    // Stop when color is black
                    if color.x.max(color.y.max(color.z)) < f32::EPSILON {
                        break;
                    }
                    // Calculate intersection
                    let t_ray = ray.clone();
                    let ray_direction = *ray.get_direction();
                    let hit_option = scene.cast_ray(&t_ray);
                    // Calculate fragment
                    if let Some(hit) = hit_option {
                        let material = &hit.object.material;

                        let bar_coords: Vector2<f32> = hit.object.bar_coords(&hit.point);
                        let uv_coors: Vector2<f32> = hit.object.uv_coords(&bar_coords);
                        let normal: Vector3<f32> = hit.object.normal(&bar_coords, &ray_direction);

                        let albedo_color: Vector3<f32> = if let Some(albedo_tex) = &material.albedo_tex {
                            f32_vector3_from_u32(albedo_tex.sample(uv_coors.x, uv_coors.y)).component_mul(&material.albedo)
                        } else {
                            material.albedo
                        };

                        ray.origin = hit.point + normal * 0.001;
                        let reflection: Vector3<f32> = reflect(ray.get_direction(), &normal);
                        let diffuse: Vector3<f32> = (normal + random_direction(&mut seed)).normalize();

                        // Calculate light contribution by explicit sampling
                        // FIXME: Produces incorrect result
                        /*let random_index = pcg::random_u32(&mut seed) as usize % scene.light_objects.len();
                        let random_light_object = &scene.objects[scene.light_objects[random_index]];
                        let light_object_midpoint: Vector3<f32> = random_light_object.random_point(&mut seed);
                        let random_ray = Ray::new(ray.origin, (light_object_midpoint - ray.origin).normalize());
                        let additional_ray = scene.cast_ray(&random_ray);

                        let additional_emission = if let Some(ar) = additional_ray {
                            if ar.object.index == hit.object.index || ar.object.index != random_light_object.index {
                                Vector3::zeros()
                            } else {
                                let ar_bar_coord: Vector2<f32> = ar.object.bar_coords(&ar.point);
                                let ar_normal: Vector3<f32> = ar.object.normal(&ar_bar_coord, random_ray.get_direction());
                                
                                ar.object.material.emission * // light emission
                                random_ray.get_direction().dot(&normal).max(0.0) * // lambert
                                random_ray.get_direction().dot(&-ar_normal).max(0.0) * // light visibility
                                (1.0 / (ar.t + 1.0).powi(2)) // Inverse square law
                            }
                        } else {
                            Vector3::zeros()
                        };*/

                        ray.set_direction(&lerp_vector3(&reflection, &diffuse, material.roughness).normalize());

                        //explicit sampling
                        /*if bounce_index == 0 {
                            light += material.emission.component_mul(&color);
                        }*/
                        light += material.emission.component_mul(&color); // comment when using explicit sampling
                        color = color.component_mul(&albedo_color);
                        //light += additional_emission.component_mul(&color); // explicit sampling
                    } else {
                        if let Some(sky_tex) = &self.texture {
                            let uvs = Self::uv_on_sphere(ray.get_direction());
                            let sky_color = sky_tex.sample(-uvs.0, uvs.1);
                            light += sky_color.component_mul(&color);
                        } else {
                            light += Vector3::new(0.3, 0.3, 0.3).component_mul(&color);
                        }
                        break;
                    }
                }

                // Blend generated pixel with old one
                let blended_color = lerp_vector3(pixel, &light, weight);
                *pixel = blended_color;
            });
        }
        self.accumulated_frames += 1;
    }

    #[inline(always)]
    fn uv_on_sphere(dir: &Vector3<f32>) -> (f32, f32) {
        let u = 0.5 + f32::atan2(dir.z, dir.x) / (2.0 * std::f32::consts::PI);
        let v = 0.5 + dir.y.asin() / (std::f32::consts::PI);
        (u.max(0.0), v.max(0.0))
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
