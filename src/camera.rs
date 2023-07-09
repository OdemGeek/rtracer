use nalgebra::{Vector3, Vector2};
use crate::math::ray::Ray;
use crate::math::extensions::euler_to_direction;

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
    pub fn new(position: Vector3<f32>, rotation: Vector3<f32>, up_vector: Vector3<f32>, fov: f32, screen_width: u16, screen_height: u16) -> Self {
        Camera { 
            position: position, rotation: rotation, direction: euler_to_direction(&rotation),
            up_vector: up_vector, fov: fov,
            screen_width: screen_width, screen_height: screen_height,
            image_distance: 0.0, forward: Vector3::<f32>::zeros(),
            right: Vector3::<f32>::zeros(), up: Vector3::<f32>::zeros() }
    }

    pub fn set_rotation(&mut self, euler_angles: Vector3<f32>) {
        self.rotation = euler_angles;
    }

    pub fn get_forward(&self) -> Vector3<f32> {
        self.forward
    }

    pub fn get_right(&self) -> Vector3<f32> {
        self.right
    }

    pub fn get_up(&self) -> Vector3<f32> {
        self.up
    }

    pub fn translate(&mut self, translation: Vector3<f32>) {
        self.position = self.position + translation;
    }

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

    pub fn ray_from_screen_point(&self, screen_pos: Vector2<f32>) -> Ray {
        let view_plane_half_height = f32::tan(self.fov / 2.0);
        let aspect_ratio = self.screen_width as f32 / self.screen_height as f32;
        let view_plane_half_width = aspect_ratio * view_plane_half_height;
        let view_plane_bottom_left_point = self.direction - self.up * view_plane_half_height - self.right * view_plane_half_width;
        let x_inc_vector = (self.right * 2.0 * view_plane_half_width) / self.screen_width as f32;
        let y_inc_vector = (self.up * 2.0 * view_plane_half_height) / self.screen_height as f32;
        let view_plane_point = view_plane_bottom_left_point + (self.screen_width as f32 - screen_pos.x) * x_inc_vector + screen_pos.y * y_inc_vector;
        let cast_ray = view_plane_point;
        Ray::new(self.position, cast_ray)
    }
}
