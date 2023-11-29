use crate::math::extensions::*;
use super::texture::{Texture, TextureSamplingMode};
use std::path::Path;
use nalgebra::Vector3;

#[allow(dead_code)]
#[inline]
fn load_image(path: &str) -> image::DynamicImage {
    image::open(Path::new(path)).expect("Failed to load image")
}

#[allow(dead_code)]
#[inline]
fn image_to_buffer(image: image::DynamicImage) -> Vec<Vector3<f32>> {
    image.to_rgb32f().pixels().map(|p| {
        let rgb = p;
        Vector3::new(rgb[0], rgb[1], rgb[2])
    }).collect()
}

#[allow(dead_code)]
#[inline]
pub fn image_to_texture(image: image::DynamicImage, sampling_mode: TextureSamplingMode) -> Texture<Vector3<f32>> {
    let buffer = image.to_rgb32f().pixels().map(|p| {
        let rgb = p;
        Vector3::new(rgb[0], rgb[1], rgb[2])
    }).collect();

    Texture::from_buffer(buffer, image.width() as usize, image.height() as usize, sampling_mode)
}

#[allow(dead_code)]
#[inline]
pub fn file_to_texture(path: &str, sampling_mode: TextureSamplingMode) -> Texture<Vector3<f32>> {
    let image = load_image(path);

    image_to_texture(image, sampling_mode)
}
