use nalgebra::Vector3;
use crate::math::ray::Ray;

pub trait Hittable{
    fn intersect(&self, ray: &Ray) -> Option<f32>;
    fn normal(&self, point: &Vector3<f32>) -> Vector3<f32>;
}