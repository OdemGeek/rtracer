use nalgebra::{Vector3, Rotation3};

pub fn euler_to_direction(euler_angles: &Vector3<f32>) -> Vector3<f32> {
    let rotation = Rotation3::from_euler_angles(euler_angles.x, euler_angles.y, euler_angles.z);
    let direction = rotation * Vector3::z();
    
    direction
}

pub fn direction_to_euler(direction: &Vector3<f32>) -> Vector3<f32> {
    let rotation = Rotation3::face_towards(&direction, &Vector3::z_axis());
    let euler_angles = rotation.euler_angles();
    
    Vector3::new(euler_angles.0, euler_angles.1, euler_angles.2)
}