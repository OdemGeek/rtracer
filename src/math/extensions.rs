use image::Rgb;
use nalgebra::{Vector3, Rotation3};

#[allow(dead_code)]
#[inline(always)]
pub fn euler_to_direction(euler_angles: &Vector3<f32>) -> Vector3<f32> {
    let rotation = Rotation3::from_euler_angles(euler_angles.x, euler_angles.y, euler_angles.z);
    rotation * Vector3::z()
}

#[allow(dead_code)]
#[inline(always)]
pub fn direction_to_euler(direction: &Vector3<f32>) -> Vector3<f32> {
    let rotation = Rotation3::face_towards(direction, &Vector3::z_axis());
    let euler_angles = rotation.euler_angles();
    
    Vector3::new(euler_angles.0, euler_angles.1, euler_angles.2)
}

// Convertions

#[allow(dead_code)]
#[inline(always)]
pub fn u8_rgb_from_u32(c: u32) -> Rgb<u8> {
    let r = ((c & 0xFF0000) >> 16) as u8;
    let g = ((c & 0x00FF00) >> 8) as u8;
    let b = (c & 0x0000FF) as u8;
    Rgb([r, g, b])
}

#[allow(dead_code)]
#[inline(always)]
pub fn u8_from_u32(c: u32) -> (u8, u8, u8) {
    let r = ((c & 0xFF0000) >> 16) as u8;
    let g = ((c & 0x00FF00) >> 8) as u8;
    let b = (c & 0x0000FF) as u8;
    (r, g, b)
}

#[allow(dead_code)]
#[inline(always)]
pub fn u32_from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

/// Returns value in 0-1 range
#[allow(dead_code)]
#[inline(always)]
pub fn f32_vector3_from_u32(c: u32) -> Vector3<f32> {
    let r = ((c & 0xFF0000) >> 16) as f32;
    let g = ((c & 0x00FF00) >> 8) as f32;
    let b = (c & 0x0000FF) as f32;
    Vector3::new(r, g, b) / 255.0
}

#[allow(dead_code)]
#[inline(always)]
pub fn reflect(incident: &Vector3<f32>, normal: &Vector3<f32>) -> Vector3<f32> {
    incident - 2.0 * incident.dot(normal) * normal
}

#[allow(dead_code)]
#[inline(always)]
pub fn lerp_vector3(a: &Vector3<f32>, b: &Vector3<f32>, t: f32) -> Vector3<f32> {
    a * (1.0 - t) + b * t
    //*pixel * (1.0 - weight) + light * weight
    // a + t * (b - a); // fast method
    // (1 - t) * a + t * b; // precise method
}