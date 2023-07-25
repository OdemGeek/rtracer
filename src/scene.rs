use crate::{math::ray::Ray, entity::{hit::Hit, hittable::Hittable, triangle::Triangle, bvh::Bvh}};

pub struct SceneData<'a> {
    pub objects: Vec<Triangle>,
    bvh: Bvh<'a>,
}

impl<'a> SceneData<'a> {
    #[inline]
    pub fn new(objects: Vec<Triangle>) -> Self {
        SceneData { objects, bvh: Bvh::new(vec![]) }
    }

    #[inline]
    pub fn add_object(&mut self, object: Triangle) -> &Triangle {
        self.objects.push(object);
        self.objects.last().unwrap()
    }

    #[inline]
    pub fn calculate_bvh(&mut self) {
        self.bvh.set_objects((0u32..self.objects.len() as u32).collect());
        self.bvh.calculate_bounds(&self.objects);
    }

    #[inline]
    pub fn cast_ray(&self, ray: &Ray) -> Option<Hit> {
        // Get closest hit
        if !self.bvh.intersect(ray) {
            return None;
        }
        self.objects.iter()
        .filter_map(|obj| {  // Take valid hits
            obj.intersect(ray)
        }) // Get min hit by param `t`
        .min_by(|hit1, hit2| hit1.t.partial_cmp(&hit2.t).unwrap())
        .map(|hit| Hit::new(
            hit.t,
            ray.origin + ray.direction * hit.t,
            hit.object.normal_flipped(&ray.direction),
            hit.object
        ))
    }
}