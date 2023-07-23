use crate::{math::ray::Ray, entity::{sphere::Sphere, hit::Hit, hittable::Hittable}};

pub struct SceneData {
    pub objects: Vec<Sphere>,
}

impl SceneData {
    #[inline]
    pub fn new(objects: Vec<Sphere>) -> Self {
        SceneData { objects: objects }
    }

    #[inline]
    pub fn add_object(&mut self, object: Sphere) -> &Sphere {
        self.objects.push(object);
        &self.objects.last().unwrap()
    }

    pub fn cast_ray(&self, ray: &Ray) -> Option<Hit> {
        // Get closest hit
        self.objects.iter()
        .filter_map(|obj| {  // Take valid hits
            obj.intersect(ray)
        }) // Get min hit by param `t`
        .min_by(|hit1, hit2| hit1.t.partial_cmp(&hit2.t).unwrap())
    }
}