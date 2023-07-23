use super::anchor::Anchor;
use super::hit::Hit;
use super::hittable::Hittable;
use crate::material::Material;
use crate::math::ray::Ray;
use std::sync::Arc;
use nalgebra::Vector3;

pub struct Sphere {
    pub anchor: Anchor,
    pub radius: f32,
    pub material: Arc<Material>,
}

impl Sphere {
    #[inline]
    pub fn new(position: Vector3<f32>, radius: f32, material: Arc<Material>) -> Self {
        Sphere { anchor: Anchor::new(position, Vector3::zeros()), radius: radius, material: material }
    }
}

impl Hittable for Sphere {
    #[inline]
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let oc = ray.origin - self.anchor.position;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None; // No intersection
        }

        let sqrt_discriminant = discriminant.sqrt();

        // Check for the closest intersection distance
        let t1 = (-b - sqrt_discriminant) / (2.0 * a);
        let t2 = (-b + sqrt_discriminant) / (2.0 * a);

        if t1 >= 0.0 && t2 >= 0.0 {
            Some(Hit::new(t1.min(t2), self))
        } else if t1 >= 0.0 {
            Some(Hit::new(t1.min(t2), self))
        } else if t2 >= 0.0 {
            Some(Hit::new(t1.min(t2), self))
        } else {
            None
        }
    }

    #[inline]
    fn normal(&self, point: &Vector3<f32>) -> Vector3<f32> {
        (point - self.anchor.position).normalize()
    }
}
