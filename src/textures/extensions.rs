use crate::math::extensions::*;
use super::texture::{Texture, TextureSamplingMode};
use std::path::Path;
use image::{Rgb, ImageResult};

#[allow(dead_code)]
#[inline]
fn load_image(path: &Path) -> ImageResult<image::DynamicImage> {
    image::open(path)
}

#[allow(dead_code)]
#[inline]
fn image_to_buffer(image: image::DynamicImage) -> Vec<u32> {
    image.to_rgb8().pixels().map(|p| {
        let rgb = p;
        u32_from_u8_rgb(rgb[0], rgb[1], rgb[2])
    }).collect()
}

#[allow(dead_code)]
#[inline]
pub fn image_to_texture(image: image::DynamicImage, sampling_mode: TextureSamplingMode) -> Texture<u32> {
    let buffer = image.to_rgb8().pixels().map(|p| {
        let rgb = p;
        u32_from_u8_rgb(rgb[0], rgb[1], rgb[2])
    }).collect();

    Texture::from_buffer(buffer, image.width() as usize, image.height() as usize, sampling_mode)
}

#[allow(dead_code)]
#[inline]
pub fn file_to_texture(path: &Path, sampling_mode: TextureSamplingMode) -> Option<Texture<u32>> {
    let image = load_image(path);
    match image {
        Ok(x) => {
            let tex = image_to_texture(x, sampling_mode);
            println!("Loaded image \"{}\". Width: {}, Height: {}", path.to_str().unwrap_or(""), tex.width(), tex.height());
            Some(tex)
        },
        Err(..) => {
            println!("Failed to load texture \"{}\"", path.to_str().unwrap_or(""));
            None
        }
    }
}

#[allow(dead_code)]
#[inline]
fn save_image_to_file(texture_buffer: &[u32], image_width: u32, image_height: u32, path: &Path) {
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
pub fn texture_to_file(texture: Texture<u32>, path: &Path) {
    save_image_to_file(
    texture.get_buffer_read(),
    texture.width() as u32,
    texture.height() as u32,
    path);
}
