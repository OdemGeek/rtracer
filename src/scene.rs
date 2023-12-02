use std::time::Instant;
use crate::{math::ray::Ray, entity::{hit::Hit, triangle::Triangle, Bounds}, bvh::{BvhNode, Bvh}};
use nalgebra::Vector3;
use rayon::prelude::*;

pub struct SceneData {
    pub objects: Vec<Triangle>,
    bvh_accel: Bvh,
    pub debug_objects: Vec<BvhNode>,
    bvh_debug: Bvh
}

impl SceneData {
    #[inline]
    pub fn new(objects: Vec<Triangle>) -> Self {
        SceneData {
            objects,
            bvh_accel: Bvh::default(),
            debug_objects: vec![],
            bvh_debug: Bvh::default(),
        }
    }

    #[inline]
    pub fn add_object(&mut self, object: Triangle) -> &Triangle {
        self.objects.push(object);
        let triangle = self.objects.last().unwrap();
        triangle
    }

    #[inline]
    pub fn calculate_bvh(&mut self) {
        let timer = Instant::now();
        let objects_bounds: Vec<Bounds> = Self::calculate_objects_bounds(&self.objects);
        println!("Bounds generation time: {} ms", timer.elapsed().as_millis());
        let objects_centroids: Vec<Vector3<f32>> = self.objects.iter().map(|x| (x.vertex1() + x.vertex2() + x.vertex3()) / 3.0).collect();
        let timer = Instant::now();
        self.bvh_accel.calculate_bvh(objects_bounds, objects_centroids);
        println!("BVH generation time: {} ms.\nBVH count: {}", timer.elapsed().as_millis(), self.bvh_accel.bvh_count());
    }

    #[inline]
    pub fn calculate_debug_bvh(&mut self, debug_depth: u32) {
        self.debug_objects = self.get_bvh_by_depth(debug_depth);
        let bvhs_bounds: Vec<Bounds> = self.debug_objects.iter().map(|x| x.into()).collect();
        let bvhs_centroids: Vec<Vector3<f32>> = bvhs_bounds.iter().map(|x| x.centroid).collect();
        self.bvh_debug.calculate_bvh(bvhs_bounds, bvhs_centroids);
    }

    #[inline]
    fn calculate_objects_bounds(objects: &[Triangle]) -> Vec<Bounds> {
        objects.par_iter().map(
            |x| x.into()
        ).collect()
    }

    #[inline]
    pub fn cast_ray<'a>(&'a self, ray: &'a Ray) -> Option<Hit<Triangle>> {
        self.bvh_accel.intersect(ray, &self.objects)
    }

    #[inline]
    pub fn cast_debug_ray<'a>(&'a self, ray: &'a Ray) -> Option<Hit<BvhNode>> {
        self.bvh_debug.intersect(ray, &self.debug_objects)
    }

    #[inline]
    pub fn get_bvh_by_depth(&self, depth: u32) -> Vec<BvhNode> {
        self.bvh_accel.get_bvh_by_depth(depth)
    }
}

