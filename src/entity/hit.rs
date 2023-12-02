use nalgebra::Vector3;
use crate::math::ray::Ray;

//use crate::math::ray::Ray;

pub struct Hit<'a, T> {
    pub t: f32,
    pub point: Vector3<f32>,
    pub object: &'a T,
}

impl<'a, T> Hit<'a, T> {
    #[inline]
    pub fn new(t: f32, point: Vector3<f32>, object: &'a T) -> Self {
        Hit { t, point, object }
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
    // distance to plane from point = n * (a - p)
    // Where n - plane normal, p - plane pos, a - point pos
    // Do this for all three points to get info about the triangle for an example
    // fn plane_cull(&self, plane: &Ray) -> bool;
}