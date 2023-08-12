use std::time::Instant;

use nalgebra::Vector3;
use crate::{math::ray::Ray, entity::{hit::Hit, hittable::Hittable, triangle::Triangle, bvh::{Bvh, BoundsTriangle}}};

pub struct SceneData {
    pub objects: Vec<Triangle>,
    pub objects_bounds: Vec<BoundsTriangle>,
    pub light_objects: Vec<u32>,
    bvhs: Vec<Bvh>,
}

impl SceneData {
    #[inline]
    pub fn new(objects: Vec<Triangle>) -> Self {
        let mut data = SceneData { objects, light_objects: vec![], objects_bounds: vec![], bvhs: vec![] };
        data.objects.iter().enumerate().for_each(|(i, x)| {
            if x.material.emission != Vector3::zeros() {
                data.light_objects.push(i as u32);
            }
        });
        data
    }

    #[inline]
    pub fn add_object(&mut self, object: Triangle) -> &Triangle {
        self.objects.push(object);
        let triangle = self.objects.last().unwrap();

        if triangle.material.emission != Vector3::zeros() {
            self.light_objects.push(self.objects.len() as u32 - 1);
        }

        triangle
    }

    #[inline]
    pub fn calculate_bvh(&mut self) {
        let timer = Instant::now();
        self.calculate_objects_bounds();
        println!("Bounds generation time: {} ms", timer.elapsed().as_millis());

        let timer = Instant::now();
        let mut bvhs_now: Vec<Bvh> = vec![Bvh::new((0u32..self.objects.len() as u32).collect())];
        let mut bvhs_next: Vec<Bvh> = vec![];
        bvhs_now[0].calculate_bounds(&self.objects);

        while !bvhs_now.is_empty() {
            for bvh in bvhs_now.iter_mut() {
                let childrens = bvh.calculate_children(&self.objects);
                if let Some(chlds) = childrens {
                    bvhs_next.push(chlds.0);
                    bvh.left_node_index = Some(bvhs_next.len() as u32 - 1);  // incorect, wrong array
                    bvhs_next.push(chlds.1);
                    bvh.right_node_index = Some(bvhs_next.len() as u32 - 1); // incorect
                }
            }
            self.bvhs.append(&mut bvhs_now.into_iter().collect());
            bvhs_now = bvhs_next.into_iter().collect();
            bvhs_next = vec![];
        }
        println!("BVH generation time: {} ms", timer.elapsed().as_millis());
        // self.bvhs.iter().enumerate().for_each(|x| println!("{}\n Left node: {}\n Right node: {} {} {}",
        // x.0, x.1.left_node_index.unwrap_or(42), x.1.right_node_index.unwrap_or(42), x.1.aabb_min, x.1.aabb_max));
    }

    #[inline]
    fn calculate_objects_bounds(&mut self) {
        self.objects_bounds = self.objects.iter().enumerate().map(
            |x| BoundsTriangle::new(
                x.0 as u32,
                x.1.vertex1(),
                x.1.vertex2(),
                x.1.vertex3(),
            )
        ).collect();
    }

    #[inline]
    pub fn cast_ray(&self, ray: &Ray) -> Option<Hit> {
        // Get closest hit
        self.bvhs[0].intersect(ray)?;
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