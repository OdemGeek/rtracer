use std::{env, vec, time::{Instant, Duration}, sync::Arc};
mod math;
mod shaders;
use material::Material;
use math::extensions::u32_from_u8_rgb;
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
mod pcg;
mod material;
use rayon::prelude::*;
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

fn print_times(accumulation_time: Duration, total_frame_elapsed: Duration,
        logic_elapsed: Duration, render_elapsed: Duration,
        window_draw_elapsed: Duration, sample_count: u32) {
    // Remove previous lines
    print!("\x1B[1A\x1B[K\x1B[1A\x1B[K\x1B[1A\x1B[K\x1B[1A\x1B[K\x1B[1A\x1B[K\x1B[1A\x1B[K\x1B[1A\x1B[K\x1B[1A\x1B[K");
    // Print new lines
    println!("Render time: {accumulation_time:.2?}\nSample count: {sample_count:?}");
    println!("Current frame timings:\nTotal: {total_frame_elapsed:.2?}");
    println!("Logic: {logic_elapsed:.2?}\nRender: {render_elapsed:.2?}\nWindow: {window_draw_elapsed:.2?}\n");
}

// TODO: Split logic of drawing screen and generating image in threads
// We shouldn't wait window to generate image
fn main() {
    let start_time = Instant::now();
    let mut imgx = 800u32;
    let mut imgy = 800u32;
    let mut max_samples = 0u32;
    
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the required number of arguments is provided
    if args.len() == 2 {
        max_samples = match args[1].parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid samples argument");
                return;
            }
        };
    } else if args.len() == 4 {
        max_samples = match args[3].parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid samples argument");
                return;
            }
        };
    }
    if args.len() >= 3 {
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
    let white_material = Arc::new(Material::new(Vector3::new(0.9, 0.9, 0.9), Vector3::zeros(), 0.99, 0.0));
    let light_material = Arc::new(Material::new(Vector3::zeros(), Vector3::new(1.0, 1.0, 1.0) * 10.0, 0.99, 0.0));
    let sphere = Sphere::new(Vector3::<f32>::new(0.0, -101.0, 4.0), 100.0, white_material.clone());
    let sphere2 = Sphere::new(Vector3::<f32>::new(0.0, 0.0, 4.0), 1.0, light_material.clone());
    let sphere3 = Sphere::new(Vector3::<f32>::new(-2.2, 0.0, 4.0), 1.0, white_material.clone());
    let sphere4 = Sphere::new(Vector3::<f32>::new(2.6, 0.5, 3.0), 1.5, white_material.clone());
    let mut scene_data = SceneData::new(vec![]);
    let _sphere_p = scene_data.add_object(sphere);
    let _sphere_p2 = scene_data.add_object(sphere2);
    let _sphere_p3 = scene_data.add_object(sphere3);
    let _sphere_p4 = scene_data.add_object(sphere4);
    
    // Setup camera
    let mut camera = Camera::new(
        Vector3::<f32>::new(0.0, 0.5, -3.0),
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
    //let texbuf: Vec<u32> = vec![0; (imgx * imgy) as usize];

    let mut render = Render::new(imgx, imgy);

    let mut frames_counted = 0;
    let mut accumulation_time = Instant::now();
    let mut counter_time = Instant::now();
    let mut frame_start = Instant::now();
    let mut total_frame_elapsed = Duration::ZERO;
    let mut logic_elapsed = Duration::ZERO;
    let mut render_elapsed = Duration::ZERO;
    let mut window_draw_elapsed = Duration::ZERO;
    let mut mouse_position = window.get_mouse_pos(minifb::MouseMode::Pass).unwrap_or((0.0, 0.0));
    let mut mouse_delta;

    let mut frame_delta = Duration::from_millis(1);
    let mut frame_index = 0u32;

    print_times(accumulation_time.elapsed(), total_frame_elapsed, logic_elapsed, render_elapsed, window_draw_elapsed, 0);
    // Loop until the window is closed
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Record frame time
        frame_delta = frame_start.elapsed();
        total_frame_elapsed += frame_delta;
        frame_start = Instant::now();
        
        // Debug frame timings
        if counter_time.elapsed().as_secs_f32() > 1.0 {
            total_frame_elapsed /= frames_counted;
            logic_elapsed /= frames_counted;
            render_elapsed /= frames_counted;
            window_draw_elapsed /= frames_counted;
            counter_time = counter_time.checked_add(Duration::from_secs(1)).unwrap_or(Instant::now());
            frames_counted = 0;
            print_times(accumulation_time.elapsed(), total_frame_elapsed, logic_elapsed, render_elapsed, window_draw_elapsed, render.get_accumulated_frames_count());
            total_frame_elapsed = Duration::ZERO;
            logic_elapsed = Duration::ZERO;
            render_elapsed = Duration::ZERO;
            window_draw_elapsed = Duration::ZERO;
        }

        // Handle input
        let mut need_to_reset = false;
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
        let move_vector_scaled = move_vector * frame_delta.as_secs_f32();

        if move_vector_scaled != Vector3::zeros() {
            camera.translate_relative(Vector3::new(-move_vector_scaled.x, move_vector_scaled.z, move_vector_scaled.y) * move_speed);
            need_to_reset |= true;
        }

        if window.get_mouse_down(minifb::MouseButton::Right) {
            if mouse_delta != Vector2::zeros() {
                camera.set_rotation(Vector3::new(mouse_delta.y * 0.002, mouse_delta.x * 0.002, 0.0) + camera.rotation);
                need_to_reset |= true;
            }
        }
        camera.init();
        // Reset frames
        if window.is_key_down(Key::R) || need_to_reset {
            render.reset_accumulated_frames();
            accumulation_time = Instant::now();
        }

        logic_elapsed += frame_start.elapsed();
        
        // Render image
        let render_start = Instant::now();
        if render.get_accumulated_frames_count() < max_samples || max_samples == 0 {
            render.draw(&scene_data, &camera);
        }
        render_elapsed += render_start.elapsed();
        
        // Draw the image in the center of the window
        let window_start = Instant::now();
        let image_u32_buffer: Vec<u32> = render.texture_buffer.par_iter().map(|p| {
            u32_from_u8_rgb(
                (p.x.powf(1.0/2.2) * 255.0) as u8,
                (p.y.powf(1.0/2.2) * 255.0) as u8,
                (p.z.powf(1.0/2.2) * 255.0) as u8
            )
        }).collect();
        window
            .update_with_buffer(&image_u32_buffer, imgx as usize, imgy as usize)
            .unwrap();
        window_draw_elapsed += window_start.elapsed();

        frame_index += 1;
        frames_counted += 1;
    }
}