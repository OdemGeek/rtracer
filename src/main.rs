use std::{env, vec, time::Instant};
mod math;
mod shaders;
use nalgebra::{Vector3, Vector2};
mod shape;
use shape::{Sphere};
mod camera;
use camera::Camera;
use minifb::{Key, Window, WindowOptions};
mod scene;
use scene::SceneData;
mod render;
use render::Render;
mod textures;
//use textures::texture::TextureSamplingMode;
//use textures::extensions::*;

#[allow(dead_code)]
fn get_current_path() -> Result<String, String> {
    if let Ok(current_dir) = env::current_dir() {
        Ok(String::from(current_dir.to_str().unwrap()))
    } else {
        Err(String::from("Failed to get current directory."))
    }
}

#[allow(dead_code)]
fn time_since_startup(start_time: Instant) -> f32 {
    Instant::now().duration_since(start_time).as_secs_f32()
}

fn main() {
    let start_time = Instant::now();

    let mut imgx = 800u32;
    let mut imgy = 800u32;

    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the required number of arguments is provided
    if args.len() == 3 {
        // Parse the arguments as integers
        imgx = match args[1].parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid width argument");
                return;
            }
        };
        imgy = match args[2].parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid height argument");
                return;
            }
        };
    }
    
    // Load skybox image
    //let skybox_texture = file_to_texture("sunset_in_the_chalk_quarry_4k.png", TextureSamplingMode::Clamp);

    // Load scene
    let sphere = Sphere::new(Vector3::<f32>::new(0.0, 0.0, 4.0), 1.0);
    let sphere2 = Sphere::new(Vector3::<f32>::new(0.0, 2.0, 4.0), 1.0);
    let mut scene_data = SceneData::new(vec![]);
    let _sphere_p = scene_data.add_object(sphere);
    let _sphere_p2 = scene_data.add_object(sphere2);
    
    // Setup camera
    let mut camera = Camera::new(
        Vector3::<f32>::zeros(),
        Vector3::<f32>::new(0.0, 0.0, 0.0),
        Vector3::<f32>::new(0.0, 1.0, 0.0),
        70.0f32.to_radians(),
        imgx as u16,
        imgy as u16);
    camera.init();
        
        
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
        
    // Create a new ImgBuf with width: imgx and height: imgy
    let texbuf: Vec<u32> = vec![0; (imgx * imgy) as usize];

    let mut render = Render::new(texbuf);

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

        // Render image
        render.draw(&scene_data, &camera);
        
        time_elapsed = time_now.elapsed();
        println!("Elapsed: {:.2?}", time_elapsed);

        // Draw the image in the center of the window
        window
            .update_with_buffer(&render.texture_buffer, imgx as usize, imgy as usize)
            .unwrap();
        
    }
}