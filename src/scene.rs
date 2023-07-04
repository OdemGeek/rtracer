use crate::{shape::Hittable, math::ray::Ray};

pub struct SceneData {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl SceneData {
    pub fn cast_ray(&self, ray: &Ray) {
        
    }
}