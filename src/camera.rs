use crate::math::ray::Ray;
use crate::math::vectors::{Float3, Float2, Vector};

pub struct Camera {
    pub position: Float3,
    pub direction: Float3,
    pub up_vector: Float3,
    pub fov: f32,
    pub screen_width: u16,
    pub screen_height: u16,
    image_distance: f32,
    forward: Float3,
    right: Float3,
    up: Float3,
}

#[allow(dead_code)]
impl Camera {
    pub fn new(position: Float3, direction: Float3, up_vector: Float3, fov: f32, screen_width: u16, screen_height: u16) -> Self {
        Camera { 
            position: position, direction: direction,
            up_vector: up_vector, fov: fov,
            screen_width: screen_width, screen_height: screen_height,
            image_distance: 0.0, forward: Float3::zero(),
            right: Float3::zero(), up: Float3::zero() }
    }

    pub fn init(&mut self) {
        self.image_distance = (self.screen_height as f32 / 2.0) / f32::tan(f32::to_radians(self.fov) / 2.0);

        self.forward = (self.direction - self.position).normalized();
        self.right = Float3::cross(&self.forward, &self.up_vector).normalized();
        self.up = Float3::cross(&self.right, &self.forward).normalized(); // wtf, why need to find up, it's already found

        print!("{:?} {:?} {:?}\n", self.forward, self.right, self.up);
    }

    pub fn ray_from_screen_point(&self, screen_pos: Float2) -> Ray {
        let view_plane_half_width = f32::tan(self.fov / 2.0);
        let aspect_ratio = self.screen_width as f32 / self.screen_height as f32;
        let view_plane_half_height = aspect_ratio * view_plane_half_width;
        let view_plane_bottom_left_point = self.direction - self.up * view_plane_half_height - self.right * view_plane_half_width;
        let x_inc_vector = (self.right * 2.0 * view_plane_half_width) / self.screen_width as f32;
        let y_inc_vector = (self.up * 2.0 * view_plane_half_height) / self.screen_height as f32;
        let view_plane_point = view_plane_bottom_left_point + (self.screen_width as f32 - screen_pos.x) * x_inc_vector + screen_pos.y * y_inc_vector;
        let cast_ray = view_plane_point - self.position;
        Ray::new(self.position, cast_ray)
    }
}
