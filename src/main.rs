use std::{path::Path, env, vec, time::Instant};
use image::Rgb;
mod math;
use math::vectors::{Float2, Float3, Vector};
mod shaders;
use shaders::{Shader, TestShader, SkyShader};
mod shape;
use shape::{Sphere, Hittable};
mod camera;
use camera::Camera;
use minifb::{Key, Window, WindowOptions};

#[allow(dead_code)]
fn load_image(path: &str) -> image::DynamicImage {
    image::open(&Path::new(path)).expect("Failed to load image")
}

#[allow(dead_code)]
fn get_current_path() -> Result<String, String> {
    if let Ok(current_dir) = env::current_dir() {
        Ok(String::from(current_dir.to_str().unwrap()))
    } else {
        Err(String::from("Failed to get current directory."))
    }
}

fn u32_from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn timeSinceStartup(startTime: Instant) -> f32 {
    Instant::now().duration_since(startTime).as_secs_f32()
}

fn main() {
    let startTime = Instant::now();
    
    let imgx = 800;
    let imgy = 800;

    // Create a new ImgBuf with width: imgx and height: imgy
    //let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    let mut texbuf: Vec<u32> = vec![0; (imgx * imgy) as usize];

    let mut camera = Camera::new(
    Float3::zero(),
    Float3::new(0.0, 0.0, 1.0),
    Float3::new(0.0, 1.0, 0.0),
    70.0,
    imgx as u16,
    imgy as u16);
    camera.init();
    
    let shape1 = Sphere::new(Float3::new(0.0, 0.0, 4.0), 1.0);
    let shape2 = Sphere::new(Float3::new(1.0, 1.5, 5.0), 1.0);

    let time_now = Instant::now();
    
    // Iterate over the coordinates and pixels of the image
    for (i, pixel) in texbuf.iter_mut().enumerate() {
        let x = i % imgx;
        let y = imgy - i / imgy;
        let screen_pos = Float2::new(x as f32 / imgx as f32, y as f32 / imgy as f32);
        // Get camera ray
        let ray = camera.ray_from_screen_point(Float2::new(x as f32, y as f32));
        // Calculate intersection
        let hit1 = shape1.intersect(&ray);
        let hit2 = shape2.intersect(&ray);
        let mut hit: Option<f32> = hit1;
        let mut shape: &Sphere = &shape1;

        if hit1.is_some() && hit2.is_some() {
            if hit1.unwrap() > hit2.unwrap() { 
                hit = hit2;
                shape = &shape2;
            } 
            else { 
                hit = hit1;
                shape = &shape1;
            };
        } else if hit1.is_some() {
            hit = hit1;
            shape = &shape1;
        } else if hit2.is_some() {
            hit = hit2;
            shape = &shape2;
        }
        let point = ray.origin + ray.direction * hit.unwrap_or(0.0);
        let normal = (point - shape.anchor.position).normalized();
        // Calculate fragment
        let color;
        if hit.is_some() {
            color = TestShader::frag(&screen_pos, &normal)
        } else {
            color = SkyShader::frag(&screen_pos, &normal)
        }
        
        // Convert Float3 to Rgb
        let final_color = color * 255.0;
        *pixel = u32_from_u8_rgb(final_color.x as u8, final_color.y as u8, final_color.z as u8);
    }

    let time_elapsed = time_now.elapsed();
    println!("Elapsed: {:.2?}", time_elapsed);



    ///////////////////
    // Draw a window //
    ///////////////////

    // Create image from texture buffer
    let imgbuf: image::ImageBuffer<Rgb<u8>, Vec<_>> = image::ImageBuffer::from_fn(imgx as u32, imgy as u32, |x, y| {
        let pixel = texbuf[(y * imgx as u32 + x) as usize];
        let r = ((pixel & 0xFF0000) >> 16) as u8;
        let g = ((pixel & 0x00FF00) >> 8) as u8;
        let b = (pixel & 0x0000FF) as u8;
        Rgb([r, g, b])
    });

    // Save generated image to file
    imgbuf.save("result.png").unwrap();
    // Open an image file using the system's default application
    //if let Err(e) = open::that("result.png") {
    //    eprintln!("Failed to open image: {}", e);
    //}

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

    // Loop until the window is closed
    while window.is_open() && !window.is_key_down(Key::Escape) {

        

        // Draw the image in the center of the window
        window
            .update_with_buffer(&texbuf, imgx as usize, imgy as usize)
            .unwrap();
    }
}