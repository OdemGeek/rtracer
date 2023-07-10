use nalgebra::{Vector3, Vector2};
use crate::math::ray::Ray;
use crate::math::extensions::euler_to_direction;
use crate::pcg::random_f32;

pub struct Camera {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    direction: Vector3<f32>,
    pub up_vector: Vector3<f32>,
    pub fov: f32,
    pub screen_width: u16,
    pub screen_height: u16,
    image_distance: f32,
    forward: Vector3<f32>,
    right: Vector3<f32>,
    up: Vector3<f32>,
}

#[allow(dead_code)]
impl Camera {
    #[inline]
    pub fn new(position: Vector3<f32>, rotation: Vector3<f32>, up_vector: Vector3<f32>, fov: f32, screen_width: u16, screen_height: u16) -> Self {
        Camera { 
            position: position, rotation: rotation, direction: euler_to_direction(&rotation),
            up_vector: up_vector, fov: fov,
            screen_width: screen_width, screen_height: screen_height,
            image_distance: 0.0, forward: Vector3::<f32>::zeros(),
            right: Vector3::<f32>::zeros(), up: Vector3::<f32>::zeros() }
    }

    #[inline]
    pub fn set_rotation(&mut self, euler_angles: Vector3<f32>) {
        self.rotation = euler_angles;
    }

    #[inline]
    pub fn get_forward(&self) -> Vector3<f32> {
        self.forward
    }

    #[inline]
    pub fn get_right(&self) -> Vector3<f32> {
        self.right
    }

    #[inline]
    pub fn get_up(&self) -> Vector3<f32> {
        self.up
    }

    #[inline]
    pub fn translate(&mut self, translation: Vector3<f32>) {
        self.position = self.position + translation;
    }

    #[inline]
    pub fn translate_relative(&mut self, translation: Vector3<f32>) {
        self.position = self.position + self.forward * translation.z + self.right * translation.x + self.up * translation.y;
    }

    pub fn init(&mut self) {
        self.image_distance = (self.screen_height as f32 / 2.0) / f32::tan(f32::to_radians(self.fov) / 2.0);

        self.direction = euler_to_direction(&self.rotation);

        self.forward = self.direction.normalize();
        self.right = self.forward.cross(&self.up_vector).normalize();
        self.up = self.right.cross(&self.forward).normalize(); // wtf, why need to find up, it's already found
    }

    #[inline]
    pub fn ray_from_screen_point(&self, screen_pos: Vector2<f32>, seed: &mut u32) -> Ray {
        let view_plane_half_height = f32::tan(self.fov / 2.0);
        let aspect_ratio = self.screen_width as f32 / self.screen_height as f32;
        let view_plane_half_width = aspect_ratio * view_plane_half_height;
        let view_plane_bottom_left_point = self.direction - self.up * view_plane_half_height - self.right * view_plane_half_width;
        let x_inc_vector = (self.right * 2.0 * view_plane_half_width) / self.screen_width as f32;
        let y_inc_vector = (self.up * 2.0 * view_plane_half_height) / self.screen_height as f32;
        let random_x_offset = random_f32(seed) - 0.5;
        let random_y_offset = random_f32(seed) - 0.5;
        let view_plane_point = view_plane_bottom_left_point
            + (self.screen_width as f32 - screen_pos.x + random_x_offset)* x_inc_vector
            + (screen_pos.y + random_y_offset) * y_inc_vector;
        let cast_ray = view_plane_point;
        Ray::new(self.position, cast_ray)
    }
}
