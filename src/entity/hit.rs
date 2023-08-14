use nalgebra::Vector3;
use super::triangle::Triangle;
use crate::math::ray::Ray;

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

#[derive(Clone)]
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

pub trait Hittable<T>{
    fn intersect(&self, ray: &Ray) -> Option<Intersection<T>>;
    fn normal(&self, ray_direction: &Vector3<f32>) -> Vector3<f32>;
    // distance to plane from point = n * (a - p)
    // Where n - plane normal, p - plane pos, a - point pos
    // Do this for all three points to get info about the triangle for an example
    // fn plane_cull(&self, plane: &Ray) -> bool;
}