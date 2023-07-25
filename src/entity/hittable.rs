use nalgebra::Vector3;
use crate::math::ray::Ray;
use super::hit::Intersection;

pub trait Hittable{
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn normal(&self, ray_direction: &Vector3<f32>) -> Vector3<f32>;
    // distance to plane from point = n * (a - p)
    // Where n - plane normal, p - plane pos, a - point pos
    // Do this for all three points to get info about the triangle for an example
    // fn plane_cull(&self, plane: &Ray) -> bool;
}