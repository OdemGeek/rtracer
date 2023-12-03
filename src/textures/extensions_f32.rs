use super::texture::{Texture, TextureSamplingMode};
use std::path::Path;
use image::ImageResult;
use nalgebra::Vector3;

#[allow(dead_code)]
#[inline]
fn load_image(path: &Path) -> ImageResult<image::DynamicImage> {
    image::open(path)
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
pub fn file_to_texture(path: &Path, sampling_mode: TextureSamplingMode) -> Option<Texture<Vector3<f32>>> {
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
