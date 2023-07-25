use nalgebra::Vector3;
use super::triangle::Triangle;

pub struct Hit<'a> {
    pub t: f32,
    pub point: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub object: &'a Triangle,
}

#[allow(dead_code)]
impl<'a> Hit<'a> {
    #[inline]
    pub fn new(t: f32, point: Vector3<f32>, normal: Vector3<f32>, object: &'a Triangle) -> Self {
        Hit { t, point, normal, object }
    }
}
