use nalgebra::Vector3;
use super::triangle::Triangle;

pub struct Hit<'a> {
    pub t: f32,
    pub point: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub object: &'a Triangle,
}

impl<'a> Hit<'a> {
    #[inline]
    pub fn new(t: f32, point: Vector3<f32>, normal: Vector3<f32>, object: &'a Triangle) -> Self {
        Hit { t, point, normal, object }
    }
}

pub struct Intersection<'a, T> {
    pub t: f32,
    pub object: &'a T,
}

impl<'a, T> Intersection<'a, T> {
    #[inline]
    pub fn new(t: f32, object: &'a T) -> Self {
        Intersection { t, object }
    }
}
