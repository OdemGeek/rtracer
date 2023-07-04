use crate::math::ray::Ray;
use nalgebra::Vector3;

pub struct Hit {
    pub t: f32,
    pub normal: Vector3<f32>,
}

#[allow(dead_code)]
impl Hit {
    pub fn new(t: f32, normal: Vector3<f32>) -> Self {
        Hit { t, normal }
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
    pub fn new(position: Vector3<f32>) -> Self {
        Anchor { position }
    }
}

pub struct Sphere {
    pub anchor: Anchor,
    pub radius: f32,
}

impl Sphere {
    pub fn new(position: Vector3<f32>, radius: f32) -> Self {
        Sphere { anchor: Anchor::new(position), radius: radius }
    }
}

impl Hittable for Sphere {
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
