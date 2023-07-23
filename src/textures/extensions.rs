use crate::math::extensions::*;
use super::texture::{Texture, TextureSamplingMode};
use std::path::Path;
use image::Rgb;

#[allow(dead_code)]
#[inline]
fn load_image(path: &str) -> image::DynamicImage {
    image::open(Path::new(path)).expect("Failed to load image")
}

#[allow(dead_code)]
#[inline]
fn save_image_to_file(texture_buffer: &[u32], image_width: u32, image_height: u32, path: &str) {
    // Create image from texture buffer
    let image_buffer: image::ImageBuffer<Rgb<u8>, Vec<_>> = image::ImageBuffer::from_fn(image_width, image_height, |x, y| {
        let pixel = texture_buffer[(y * image_width + x) as usize];
        u8_rgb_from_u32(pixel)
    });

    // Save generated image to file
    image_buffer.save(path).unwrap();
}

#[allow(dead_code)]
#[inline]
fn image_to_buffer(image: image::DynamicImage) -> Vec<u32> {
    image.to_rgb8().pixels().map(|p| {
        let rgb = p.0;
        u32_from_u8_rgb(rgb[0], rgb[1], rgb[2])
    }).collect()
}

#[allow(dead_code)]
#[inline]
pub fn image_to_texture(image: image::DynamicImage, sampling_mode: TextureSamplingMode) -> Texture {
    let buffer = image.to_rgb8().pixels().map(|p| {
        let rgb = p.0;
        u32_from_u8_rgb(rgb[0], rgb[1], rgb[2])
    }).collect();

    Texture::from_buffer(buffer, image.width() as u16, image.height() as u16, sampling_mode)
}

#[allow(dead_code)]
#[inline]
pub fn file_to_texture(path: &str, sampling_mode: TextureSamplingMode) -> Texture {
    let image = load_image(path);

    let buffer = image.to_rgb8().pixels().map(|p| {
        let rgb = p.0;
        u32_from_u8_rgb(rgb[0], rgb[1], rgb[2])
    }).collect();

    Texture::from_buffer(buffer, image.width() as u16, image.height() as u16, sampling_mode)
}

#[allow(dead_code)]
#[inline]
pub fn texture_to_file(texture: Texture, path: &str) {
    save_image_to_file(
    texture.get_buffer_read(),
    texture.width() as u32,
    texture.height() as u32,
    path);
}
