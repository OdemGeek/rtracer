use crate::{math::ray::Ray, entity::{hit::Hit, hittable::Hittable, triangle::Triangle}};

pub struct SceneData {
    pub objects: Vec<Triangle>,
}

impl SceneData {
    #[inline]
    pub fn new(objects: Vec<Triangle>) -> Self {
        SceneData { objects }
    }

    #[inline]
    pub fn add_object(&mut self, object: Triangle) -> &Triangle {
        self.objects.push(object);
        self.objects.last().unwrap()
    }

    #[inline]
    pub fn cast_ray(&self, ray: &Ray) -> Option<Hit> {
        // Get closest hit
        self.objects.iter()
        .filter_map(|obj| {  // Take valid hits
            obj.intersect(ray)
        }) // Get min hit by param `t`
        .min_by(|hit1, hit2| hit1.t.partial_cmp(&hit2.t).unwrap())
    }
}