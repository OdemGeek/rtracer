use std::{path::Path, env, vec, time::Instant};
use image::Rgb;
mod math;
mod shaders;
use nalgebra::{Vector3, Vector2};
use shaders::{Shader, TestShader, SkyShader};
mod shape;
use shape::{Sphere, Hittable};
mod camera;
use camera::Camera;
use minifb::{Key, Window, WindowOptions};
mod scene;
mod render;
use rayon::prelude::*;

#[allow(dead_code)]
fn load_image(path: &str) -> image::DynamicImage {
    image::open(&Path::new(path)).expect("Failed to load image")
}

#[allow(dead_code)]
fn image_to_buffer(image: image::DynamicImage) -> Vec<u32> {
    image.to_rgb8().pixels().map(|p| {
        let rgb = p.0;
        u32_from_u8_rgb(rgb[0], rgb[1], rgb[2])
    }).collect()
}

#[allow(dead_code)]
fn save_image_to_file(texture_buffer: Vec<u32>, image_width: u32, image_height: u32, path: &str){
    // Create image from texture buffer
    let image_buffer: image::ImageBuffer<Rgb<u8>, Vec<_>> = image::ImageBuffer::from_fn(image_width, image_height, |x, y| {
        let pixel = texture_buffer[(y * image_width + x) as usize];
        u8_rgb_from_u32(pixel)
    });

    // Save generated image to file
    image_buffer.save(path).unwrap();
    // Open an image file using the system's default application
    if let Err(e) = open::that(path) {
        eprintln!("Failed to open image: {}", e);
    }
}

#[allow(dead_code)]
fn get_current_path() -> Result<String, String> {
    if let Ok(current_dir) = env::current_dir() {
        Ok(String::from(current_dir.to_str().unwrap()))
    } else {
        Err(String::from("Failed to get current directory."))
    }
}

#[allow(dead_code)]
fn u8_rgb_from_u32(c: u32) -> Rgb<u8> {
    let r = ((c & 0xFF0000) >> 16) as u8;
    let g = ((c & 0x00FF00) >> 8) as u8;
    let b = (c & 0x0000FF) as u8;
    Rgb([r, g, b])
}

fn u32_from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn f32_vector3_from_u32(c: u32) -> Vector3<f32> {
    let r = ((c & 0xFF0000) >> 16) as f32;
    let g = ((c & 0x00FF00) >> 8) as f32;
    let b = (c & 0x0000FF) as f32;
    Vector3::new(r, g, b)
}

#[allow(dead_code)]
fn time_since_startup(start_time: Instant) -> f32 {
    Instant::now().duration_since(start_time).as_secs_f32()
}


fn main() {
    let start_time = Instant::now();
    
    let imgx = 800;
    let imgy = 800;

    let skybox_image = load_image("sunset_in_the_chalk_quarry_4k.png");
    let skybox_dimensions = (skybox_image.width(), skybox_image.height());
    let skybox = image_to_buffer(skybox_image);

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut texbuf: Vec<u32> = vec![0; (imgx * imgy) as usize];
    
    let mut camera = Camera::new(
    Vector3::<f32>::zeros(),
    Vector3::<f32>::new(0.0, 0.0, 0.0),
    Vector3::<f32>::new(0.0, 1.0, 0.0),
    70.0,
    imgx as u16,
    imgy as u16);
    camera.init();

    let shape = Sphere::new(Vector3::<f32>::new(0.0, 0.0, 4.0), 1.0);

    // Create a window with the specified dimensions
    let mut window = Window::new(
        "Rust Window",
        imgx as usize,
        imgy as usize,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut time_elapsed = start_time.elapsed();
    let mut mouse_position = window.get_mouse_pos(minifb::MouseMode::Pass).unwrap_or((0.0, 0.0));
    let mut mouse_delta;
    // Loop until the window is closed
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let time_now = Instant::now();

        // Handle input
        // Mouse
        let current_mouse_pos = window.get_mouse_pos(minifb::MouseMode::Pass).unwrap_or(mouse_position);
        mouse_delta = Vector2::new(current_mouse_pos.0 - mouse_position.0, current_mouse_pos.1 - mouse_position.1);
        mouse_position = current_mouse_pos;
        // Keyboard
        // Move speed
        let move_speed = if window.is_key_down(Key::LeftShift) {10.0} else {3.0};
        // Move vector
        let right = window.is_key_down(Key::D);
        let left = window.is_key_down(Key::A);
        let forward = window.is_key_down(Key::W);
        let back = window.is_key_down(Key::S);
        let up = window.is_key_down(Key::E);
        let down = window.is_key_down(Key::Q);
        let move_vector = Vector3::new(
            (right as i32 - left as i32) as f32,
            (forward as i32 - back as i32) as f32,
            (up as i32 - down as i32) as f32);
        let move_vector_scaled = move_vector * time_elapsed.as_secs_f32();

        camera.translate_relative(Vector3::new(-move_vector_scaled.x, move_vector_scaled.z, move_vector_scaled.y) * move_speed);

        if window.get_mouse_down(minifb::MouseButton::Right) {
            camera.set_rotation(Vector3::new(mouse_delta.y * 0.002, mouse_delta.x * 0.002, 0.0) + camera.rotation);
        }
        camera.init();

        // Iterate over the pixels of the image
        texbuf.par_iter_mut().enumerate().for_each(|(i, pixel)| {
            let x = i % imgx;
            let y = imgy - i / imgy;
            let screen_pos = Vector2::<f32>::new(x as f32 / imgx as f32, y as f32 / imgy as f32);
            // Get camera ray
            let ray = camera.ray_from_screen_point(Vector2::<f32>::new(x as f32, y as f32));
            // Calculate intersection
            let hit = shape.intersect(&ray);

            // Calculate fragment
            let color;
            if hit.is_some() {
                let point = ray.origin + ray.direction * hit.unwrap_or(1.0);
                let normal = (point - shape.anchor.position).normalize();
                color = TestShader::frag(&screen_pos, &normal, &skybox, &skybox_dimensions);
            } else {
                color = SkyShader::frag(&screen_pos, &ray.direction, &skybox, &skybox_dimensions);
            }
            
            // Convert Float3 to Rgb
            let final_color = color * 255.0;
            *pixel = u32_from_u8_rgb(final_color.x as u8, final_color.y as u8, final_color.z as u8);
        });
        time_elapsed = time_now.elapsed();
        println!("Elapsed: {:.2?}", time_elapsed);
        // Draw the image in the center of the window
        window
            .update_with_buffer(&texbuf, imgx as usize, imgy as usize)
            .unwrap();
        
    }
}