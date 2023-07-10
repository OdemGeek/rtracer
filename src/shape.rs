use std::sync::Arc;
use crate::math::ray::Ray;
use crate::material::Material;
use nalgebra::Vector3;

pub struct Hit<'a> {
    pub t: f32,
    pub object: &'a Sphere,
}

#[allow(dead_code)]
impl<'a> Hit<'a> {
    pub fn new(t: f32, object: &'a Sphere) -> Self {
        Hit { t, object }
    }
}

pub trait Hittable{
    //fn intersect(&self, ray: &Ray) -> Hit;
    fn intersect(&self, ray: &Ray) -> Option<f32>;
}

pub struct Anchor {
    pub position: Vector3<f32>
}

impl Anchor {
    #[inline]
    pub fn new(position: Vector3<f32>) -> Self {
        Anchor { position }
    }
}

pub struct Sphere {
    pub anchor: Anchor,
    pub radius: f32,
    pub material: Arc<Material>,
}

impl Sphere {
    #[inline]
    pub fn new(position: Vector3<f32>, radius: f32, material: Arc<Material>) -> Self {
        Sphere { anchor: Anchor::new(position), radius: radius, material: material }
    }
}

impl Hittable for Sphere {
    #[inline]
    fn intersect(&self, ray: &Ray) -> Option<f32> {
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
            Some(t1.min(t2))
        } else if t1 >= 0.0 {
            Some(t1)
        } else if t2 >= 0.0 {
            Some(t2)
        } else {
            None
        }
    }
}
