use nalgebra::Vector3;
use crate::math::ray::Ray;
use super::hit::Hit;

pub trait Hittable{
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
    fn normal(&self, point: &Vector3<f32>) -> Vector3<f32>;
}