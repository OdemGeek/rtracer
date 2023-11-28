use crate::{math::ray::Ray, entity::{hit::Hit, triangle::Triangle, Bounds}, bvh::{BvhNode, Bvh}};

pub struct SceneData {
    pub objects: Vec<Triangle>,
    bvh_accel: Bvh
}

impl SceneData {
    #[inline]
    pub fn new(objects: Vec<Triangle>) -> Self {
        SceneData {
            objects,
            bvh_accel: Bvh::default()
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn add_object(&mut self, object: Triangle) -> &Triangle {
        self.objects.push(object);
        let triangle = self.objects.last().unwrap();
        triangle
    }

    #[inline]
    pub fn calculate_bvh(&mut self) {
        let objects_bounds: Vec<Bounds> = Self::calculate_objects_bounds(&self.objects);
        self.bvh_accel.calculate_bvh(objects_bounds);
    }

    #[inline]
    fn calculate_objects_bounds(objects: &[Triangle]) -> Vec<Bounds> {
        objects.iter().enumerate().map(
            |x| Bounds::new(
                x.1.vertex1(),
                x.1.vertex2(),
                x.1.vertex3(),
            )
        ).collect()
    }

    #[inline]
    pub fn cast_ray(&self, ray: &Ray) -> Option<Hit<Triangle>> {
        self.bvh_accel.intersect(ray, &self.objects)
    }

    #[inline]
    pub fn get_bvh_by_depth(&self, depth: u32) -> Vec<&BvhNode> {
        self.bvh_accel.get_bvh_by_depth(depth)
    }
}

