use nalgebra::{Vector3, Rotation3};
// nalgebra extensions

pub fn euler_to_direction(euler_angles: Vector3<f32>) -> Vector3<f32> {
    let rotation = Rotation3::from_euler_angles(euler_angles.x, euler_angles.y, euler_angles.z);
    let direction = rotation * Vector3::z();

    direction
}
