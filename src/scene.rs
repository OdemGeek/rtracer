use crate::{shape::{Hittable, Hit, Sphere}, math::ray::Ray};

pub struct SceneData {
    pub objects: Vec<Sphere>,
}

impl SceneData {
    pub fn new(objects: Vec<Sphere>) -> Self {
        SceneData { objects: objects }
    }

    pub fn add_object(&mut self, object: Sphere) -> &Sphere {
        self.objects.push(object);
        &self.objects.last().unwrap()
    }

    // TODO: Optimize condition and option logic
    // For an example we don't need to return hit in obj.intersect()
    // We should create it here
    pub fn cast_ray(&self, ray: &Ray) -> Option<Hit> {
        let mut closest_hit: f32 = f32::INFINITY;
        let mut closest_obj: Option<&Sphere> = None;
        for obj in self.objects.iter() {
            let hit_check = obj.intersect(ray);
            // Skip if we didn't hit
            if (hit_check).is_none() {
                continue;
            }
            let hit = hit_check.unwrap();

            if hit < closest_hit {
                closest_hit = hit;
                closest_obj = Some(&obj);
            }
        }
        if closest_obj.is_none() {
            None
        } else {
            Some(Hit::new(closest_hit, &closest_obj.unwrap()))
        }
    }
}