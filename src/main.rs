use std::{env, time::{Instant, Duration}, sync::{Arc, Mutex, Condvar, atomic::AtomicBool}};
use std::thread;
use std::sync::atomic::Ordering;
use rtracer::math::extensions::u32_from_u8_rgb;
use rtracer::textures::texture::TextureSamplingMode;
use rtracer::textures::extensions_f32::file_to_texture;
use nalgebra::{Vector3, Vector2};
use rtracer::camera::Camera;
use minifb::{Key, Window, WindowOptions};
use rtracer::scene::SceneData;
use rtracer::render::Render;
use rtracer::loaders::scene_loader::load_scene;

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
#[inline]
fn time_since_startup(start_time: Instant) -> f32 {
    Instant::now().duration_since(start_time).as_secs_f32()
}

// REWRITE THIS TO SEPERATE SYSTEM!
#[allow(clippy::too_many_arguments)]
fn print_times(accumulation_time: Duration, total_frame_elapsed: Duration,
        main_thread_wait_elapsed: Duration, render_thread_wait_elapsed: Duration,
        logic_elapsed: Duration, render_elapsed: Duration,
        window_draw_elapsed: Duration, sample_count: u32,
        clear_console: bool) {
    // Remove previous lines
    if clear_console {
        // Let's not talk about it
        for _ in 0..10 {
            print!("\x1B[1A\x1B[K");
        }
    }
    // Print new lines
    println!("Render time: {accumulation_time:.2?}\nSample count: {sample_count:?}");
    println!("Current frame timings:\nTotal: {total_frame_elapsed:.2?}");
    println!("Render: {render_elapsed:.2?}\nWait for main thread: {render_thread_wait_elapsed:.2?}");
    println!("Wait for render thread: {main_thread_wait_elapsed:.2?}\nWindow: {window_draw_elapsed:.2?}\nLogic: {logic_elapsed:.2?}\n");
}

struct RenderData {
    render: Render,
    updated: bool,
    max_samples: u32,
    scene_data: SceneData,
    camera: Camera,
    accumulated_time: Duration,
    accumulation_time: Instant,
    render_elapsed: Duration,
    render_wait_elapsed: Duration,
    render_frames_counted: u32,
}

impl RenderData {
    #[inline]
    pub fn draw(&mut self) {
        self.render.draw(&self.scene_data, &self.camera);
    }
}

fn print_help() {
    println!("
    █▀▀█ ▀▀█▀▀ █▀▀█ █▀▀█ █▀▀ █▀▀ █▀▀█ 
    █▄▄▀   █   █▄▄▀ █▄▄█ █   █▀▀ █▄▄▀ 
    █  █   █   ▀ ▀▀ ▀  ▀ ▀▀▀ ▀▀▀ ▀ ▀▀
                                by OdemGeek

Argument syntax:
    [help]
    [-SampleCount]
    [-ImageWidth] [-ImageHeight]
    [-ImageWidth] [-ImageHeight] [-SampleCount]

    If no arguments passed SampleCount is set to 0, ImageWidth and ImageHeight is 800
    If SampleCount is 0, render is infinite.
         ");
}


// For some reason image crate can't load .hdr files with correct exposure
// So if you want high dinamic range please use .exr
fn main() {
    let _start_time = Instant::now();
    let mut imgx = 800u32;
    let mut imgy = 800u32;
    let mut max_samples = 0u32;

    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Print help text
    if args.len() == 2 && args[1].to_lowercase() == "help" {
        print_help();
        return;
    }
    
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


    // Create scene
    let (loaded_geometry, mut camera) = load_scene("scene.rts");
    let mut scene_data = SceneData::new(loaded_geometry);
    println!("Triangle count: {}", scene_data.objects.len());
    // Calculate bvh for loaded scene
    scene_data.calculate_bvh();

    // Load skybox image
    let skybox_texture = file_to_texture("sunset_in_the_chalk_quarry_4k.exr", TextureSamplingMode::Clamp);
    println!("Sky width: {}", skybox_texture.width());
    println!("Sky height: {}", skybox_texture.height());

    // Setup camera
    camera.screen_width = imgx as u16;
    camera.screen_height = imgy as u16;
    camera.init();
    
    // Create a window with the specified dimensions
    let mut window = Window::new(
        "RTracer",
        imgx as usize,
        imgy as usize,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    // Limit window fps to 120
    window.limit_update_rate(Some(Duration::from_secs_f32(1.0 / 120.0)));

    let render = Render::new(imgx, imgy, skybox_texture);
    let accumulation_time = Instant::now();
    let accumulated_time = Duration::ZERO;
    let render_elapsed = Duration::ZERO;
    let render_wait_elapsed = Duration::ZERO;

    let mut frames_counted = 0;
    let mut counter_time = Instant::now();
    let mut frame_start = Instant::now();
    let mut logic_start;
    let mut thread_wait;
    let mut total_frame_elapsed = Duration::ZERO;
    let mut logic_elapsed = Duration::ZERO;
    let mut main_thread_wait_elapsed = Duration::ZERO;
    let mut window_draw_elapsed = Duration::ZERO;
    let mut mouse_position = window.get_mouse_pos(minifb::MouseMode::Pass).unwrap_or((0.0, 0.0));
    let mut mouse_delta;
    let mut frame_delta;

    // Create a mutex to protect the shared data
    let mut output_buffer = render.texture_buffer.clone();
    let render_data = Arc::new(Mutex::new(
        RenderData {
            render, updated: true, max_samples,
            scene_data, camera, accumulated_time,
            accumulation_time, render_elapsed,
            render_wait_elapsed, render_frames_counted: 0,
    }));
    let pause = Arc::new(AtomicBool::new(false));
    let stop = Arc::new(AtomicBool::new(false));

    // Create a condition variable to signal the thread to resume
    let condvar = Arc::new(Condvar::new());

    println!();
    print_times(accumulation_time.elapsed(), total_frame_elapsed, main_thread_wait_elapsed, render_wait_elapsed, logic_elapsed, render_elapsed, window_draw_elapsed, 0, false);
    
    // Clone references to the mutex and condition variable for the thread to use
    let thread_render_data = Arc::clone(&render_data);
    let thread_condvar = Arc::clone(&condvar);
    let thread_pause = Arc::clone(&pause);
    let thread_stop = Arc::clone(&stop);

    let render_thread = thread::spawn(move || {
        // Access the shared data via the mutex
        let mut data = thread_render_data.lock().unwrap();
        
        loop {
            let wait_start = Instant::now();
            // Check if the thread is paused
            while thread_pause.load(Ordering::Relaxed) {
                data = thread_condvar.wait(data).unwrap();
            }
            // Kill loop if need to stop thread
            if thread_stop.load(Ordering::Relaxed) {
                break;
            }
            data.render_wait_elapsed += wait_start.elapsed();

            let render_start = Instant::now();
            // Perform your thread's logic here, using the shared data
            // Render image
            if data.render.get_accumulated_frames_count() < data.max_samples || data.max_samples == 0 {
                data.draw();
                data.accumulated_time = data.accumulation_time.elapsed();
                data.updated = true;
            }
            data.render_elapsed += render_start.elapsed();
            data.render_frames_counted += 1;
        }
    });
    
    let mut image_updated;
    let mut image_u32_buffer: Vec<u32> = output_buffer.iter().map(|p| {
        u32_from_u8_rgb(
            (p.x * 255.0) as u8,
            (p.y * 255.0) as u8,
            (p.z * 255.0) as u8
        )
    }).collect();
    let mut debug_depth: u32 = 0;
    // Loop until the window is closed
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Record frame time
        frame_delta = frame_start.elapsed();
        total_frame_elapsed += frame_delta;
        frame_start = Instant::now();
        
        {
            thread_wait = Instant::now();
            pause.store(true, Ordering::Relaxed);
            let mut data = render_data.lock().unwrap();
            main_thread_wait_elapsed += thread_wait.elapsed();
            
            logic_start = Instant::now();
            image_updated = data.updated;
            // Debug frame timings
            if counter_time.elapsed().as_secs_f32() > 1.0 && frames_counted > 0 {
                total_frame_elapsed /= frames_counted;
                main_thread_wait_elapsed /= frames_counted;
                logic_elapsed /= frames_counted;
                let render_frames_counted = data.render_frames_counted;
                if render_frames_counted > 0 {
                    data.render_elapsed /= render_frames_counted;
                    data.render_wait_elapsed /= render_frames_counted;
                }
                window_draw_elapsed /= frames_counted;
                counter_time = counter_time.checked_add(Duration::from_secs(1)).unwrap_or(Instant::now());
                frames_counted = 0;
                data.render_frames_counted = 0;
                print_times(data.accumulated_time, total_frame_elapsed, main_thread_wait_elapsed, data.render_wait_elapsed, logic_elapsed, data.render_elapsed, window_draw_elapsed, data.render.get_accumulated_frames_count(), true);
                total_frame_elapsed = Duration::ZERO;
                data.render_wait_elapsed = Duration::ZERO;
                main_thread_wait_elapsed = Duration::ZERO;
                logic_elapsed = Duration::ZERO;
                data.render_elapsed = Duration::ZERO;
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
                data.camera.anchor.translate_relative(Vector3::new(move_vector_scaled.x, move_vector_scaled.z, move_vector_scaled.y) * move_speed);
                need_to_reset = true;
            }
            
            let mut update_debug_bvh = false;
            if window.is_key_pressed(Key::I, minifb::KeyRepeat::No) {
                data.render.bvh_debug = !data.render.bvh_debug;
                data.max_samples = if data.render.bvh_debug {
                    16
                } else {
                    max_samples
                };
                need_to_reset = true;
                update_debug_bvh = true;
            }
            if data.render.bvh_debug {
                if window.is_key_pressed(Key::Period, minifb::KeyRepeat::No) {
                    debug_depth += 1;
                    need_to_reset = true;
                    update_debug_bvh = true;
                }
                if window.is_key_pressed(Key::Comma, minifb::KeyRepeat::No) && debug_depth > 0 {
                    debug_depth -= 1;
                    need_to_reset = true;
                    update_debug_bvh = true;
                }
            }

            if update_debug_bvh {
                data.scene_data.calculate_debug_bvh(debug_depth);
            }

            if window.get_mouse_down(minifb::MouseButton::Right) && mouse_delta != Vector2::zeros() {
                let rotation = data.camera.anchor.rotation();
                data.camera.anchor.set_rotation(Vector3::new(mouse_delta.y * 0.002, mouse_delta.x * 0.002, 0.0) + rotation);
                need_to_reset = true;
            }
            data.camera.init();
            // Reset frames
            if window.is_key_down(Key::R) || need_to_reset {
                data.render.reset_accumulated_frames();
                data.accumulation_time = Instant::now();
            }

            if image_updated {
                output_buffer.clone_from_slice(&data.render.texture_buffer);
            }

            data.updated = false;
            pause.store(false, Ordering::Relaxed);
            condvar.notify_one();
            logic_elapsed += logic_start.elapsed();
        }

        // Draw the image in the center of the window
        let window_start = Instant::now();
        if image_updated {
            image_u32_buffer = output_buffer.iter().map(|p| {
                u32_from_u8_rgb(
                    (p.x.powf(1.0/2.2) * 255.0) as u8,
                    (p.y.powf(1.0/2.2) * 255.0) as u8,
                    (p.z.powf(1.0/2.2) * 255.0) as u8
                )
            }).collect();
        }
        window
            .update_with_buffer(&image_u32_buffer, imgx as usize, imgy as usize)
            .unwrap();
        window_draw_elapsed += window_start.elapsed();

        frames_counted += 1;
    }

    stop.store(true, Ordering::Relaxed);
    render_thread.join().unwrap();
}
