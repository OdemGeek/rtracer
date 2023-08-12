use std::{fs::File, io::{BufReader, prelude::*}};
use crate::{entity::triangle::Triangle, camera::Camera};
use super::model_loader;
use model_loader::load_model;
use nalgebra::Vector3;

pub fn load_scene(path: &str) -> (Vec<Triangle>, Camera) {
    let file = File::open(path).unwrap_or_else(|_|
        panic!("Failed to load scene file \"{}\". Reason: Not found", &path)
    );
    let reader = BufReader::new(file);
    let mut models: Vec<String> = vec![];
    let mut camera = Camera::new(
        Vector3::new(0.0, 1.0, 3.0),
        Vector3::new(0.0, 180.0f32.to_radians(), 0.0),
        70.0f32.to_radians(),
        800,
        800
    );
    
    let mut is_reading_model = false;
    let mut is_reading_camera = false;
    reader.lines().map_while(Result::ok).filter(|x| !(x.starts_with('#') || x.is_empty())).for_each(|ref x| {
        match x.to_lowercase().as_str() {
            "model[" => is_reading_model = true,
            "]model" =>  is_reading_model = false,
            "camera[" =>  is_reading_camera = true,
            "]camera" =>  is_reading_camera = false,
            _ => {
                if is_reading_model && x.ends_with(".obj") {
                    models.push(x.clone());
                }
                if is_reading_camera {
                    let s: Vec<&str> = x.split_whitespace().collect();
                    match s.first() {
                        Some(&"p") => {
                            let mut vector = Vector3::zeros();
                            if let Some(n) = s.get(1) {
                                vector.x = n.parse::<f32>()
                                    .expect("Failed to read X coordinate of the camera.");
                            }
                            if let Some(n) = s.get(2) {
                                vector.y = n.parse::<f32>()
                                    .expect("Failed to read Y coordinate of the camera.");
                            }
                            if let Some(n) = s.get(3) {
                                vector.z = n.parse::<f32>()
                                    .expect("Failed to read Z coordinate of the camera.");
                            }
                            camera.anchor.set_position(vector);
                        },
                        Some(&"r") => {
                            let mut vector = Vector3::zeros();
                            if let Some(n) = s.get(1) {
                                vector.x = n.parse::<f32>()
                                    .expect("Failed to read X coordinate of the camera.")
                                    .to_radians();
                            }
                            if let Some(n) = s.get(2) {
                                vector.y = n.parse::<f32>()
                                    .expect("Failed to read Y coordinate of the camera.")
                                    .to_radians();
                            }
                            if let Some(n) = s.get(3) {
                                vector.z = n.parse::<f32>()
                                    .expect("Failed to read Z coordinate of the camera.")
                                    .to_radians();
                            }
                            camera.anchor.set_rotation(vector);
                        },
                        Some(&"f") => {
                            if let Some(n) = s.get(1) {
                                let fov = n.parse::<f32>()
                                    .expect("Failed to read FoV of the camera.")
                                    .to_radians();
                                camera.fov = fov;
                            }
                        },
                        _ => ()
                    }
                }
            }
        }
    });

    println!("{:?}", models);
    let mut triangles: Vec<Triangle> = Vec::new();
    for m in models.iter() {
        triangles.extend(load_model(m));
    }
    (triangles, camera)
}