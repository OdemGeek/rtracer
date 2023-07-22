use nalgebra::{Vector3, Vector2};
use crate::entity::anchor::Anchor;
use crate::math::ray::Ray;
use crate::math::pcg::random_f32;

pub struct Camera {
    pub anchor: Anchor,
    pub fov: f32,
    pub screen_width: u16,
    pub screen_height: u16,
    image_distance: f32,
}

#[allow(dead_code)]
impl Camera {
    #[inline]
    pub fn new(position: Vector3<f32>, rotation: Vector3<f32>, fov: f32, screen_width: u16, screen_height: u16) -> Self {
        Camera {
            anchor: Anchor::new(position, rotation),
            fov: fov,
            screen_width: screen_width,
            screen_height: screen_height,
            image_distance: 0.0,
        }
    }

    pub fn init(&mut self) {
        self.image_distance = (self.screen_height as f32 / 2.0) / f32::tan(f32::to_radians(self.fov) / 2.0);
    }

    #[inline]
    pub fn ray_from_screen_point(&self, screen_pos: Vector2<f32>, seed: &mut u32) -> Ray {
        let view_plane_half_height = f32::tan(self.fov / 2.0);
        let aspect_ratio = self.screen_width as f32 / self.screen_height as f32;
        let view_plane_half_width = aspect_ratio * view_plane_half_height;
        let view_plane_bottom_left_point =
            self.anchor.forward().into_inner() 
            - self.anchor.up().into_inner() * view_plane_half_height 
            - self.anchor.right().into_inner() * view_plane_half_width;
        
        let x_inc_vector = (self.anchor.right().into_inner() * 2.0 * view_plane_half_width) / self.screen_width as f32;
        let y_inc_vector = (self.anchor.up().into_inner() * 2.0 * view_plane_half_height) / self.screen_height as f32;
        let random_x_offset = random_f32(seed) - 0.5;
        let random_y_offset = random_f32(seed) - 0.5;
        let view_plane_point = view_plane_bottom_left_point
            + (screen_pos.x + random_x_offset) * x_inc_vector
            + (screen_pos.y + random_y_offset) * y_inc_vector;
        let cast_ray = view_plane_point;
        Ray::new(self.anchor.position, cast_ray)
    }
}
