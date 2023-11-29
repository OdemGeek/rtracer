mod bvh_intersection;
mod bvh_depth;
mod bvh_node;
use bvh_intersection::BvhIntersection;
pub use bvh_depth::BvhDepth;
pub use bvh_node::BvhNode;

use std::time::Instant;
use nalgebra::Vector3;
use crate::math::ray::Ray;
use crate::entity::hit::{Hit, Hittable};
use crate::entity::Bounds;

pub struct Bvh {
    pub(super) bvhs: Vec<BvhNode>,
    pub(super) objects_bounds: Vec<Bounds>,
    pub(super) objects_indexes: Vec<usize>,
    nodes_used: usize
}

impl Default for Bvh {
    #[inline]
    fn default() -> Self {
        Bvh {
            bvhs: vec![],
            objects_bounds: vec![],
            objects_indexes: vec![],
            nodes_used: 0
        }
    }
}

impl Bvh {
    #[inline]
    pub fn intersect<'a, T>(&'a self, ray: &Ray, objects: &'a [T]) -> Option<Hit<T>>
    where T: Hittable<T> {
        // Get closest hit
        let mut bvh_intersection = BvhIntersection::new(self);
        bvh_intersection.intersect_hierarchy(ray, objects);

        bvh_intersection.closest_hit.map(move |hit| Hit::new(
            hit.t,
            ray.origin + ray.direction * hit.t,
            hit.object.normal(&ray.direction),
            hit.object
        ))
    }

    #[inline]
    pub fn get_bvh_by_depth(&self, depth: u32) -> Vec<&BvhNode> {
        let mut bd = BvhDepth::new(&self.bvhs, depth);
        bd.intersect_hierarchy();
        bd.bvhs
    }

    #[inline]
    pub fn calculate_bvh(&mut self, objects_bounds: Vec<Bounds>) {
        self.objects_indexes = (0..objects_bounds.len()).collect();
        self.objects_bounds = objects_bounds;


        let timer = Instant::now();

        self.bvhs = Vec::with_capacity(self.objects_bounds.len() * 2 - 1);
        self.bvhs.resize(self.bvhs.capacity(), BvhNode::new(0, 0));

        self.nodes_used = 1;

        let root_bvh = &mut self.bvhs[0];
        root_bvh.first_object = 0;
        root_bvh.object_count = self.objects_bounds.len();
        self.calculate_bvh_bounds(0);
        

        let mut current_index = 0;

        while current_index < self.nodes_used {
            self.calculate_childs(current_index);
            current_index += 1;
        }

        self.bvhs.resize(self.nodes_used, BvhNode::new(0, 0));

        //println!("BVH generation time: {} ms", timer.elapsed().as_millis());
        /*self.bvhs.iter().enumerate().for_each(|x| {
            println!("{} {:?}\n", x.0, x.1);
        });

        let mut current_depth = 0;
        loop {
            let current_count = self.get_bvh_by_depth(current_depth).len();
            if current_count == 0 {
                break;
            }
            println!("Depth {}. Count: {}", current_depth, current_count);
            current_depth += 1;
        }*/
    }

    #[inline]
    fn calculate_childs(&mut self, bvh_index: usize) {
        let s: Option<(Vector3<f32>, Vector3<f32>)> = self.calculate_bvh_centroids(bvh_index);
        let bvh = &mut self.bvhs[bvh_index];
        if bvh.object_count < 3 {
            return;
        }
        let s: (Vector3<f32>, Vector3<f32>) = s.unwrap();
        let (split_pos, divide_axis) = BvhNode::division_plane(s.0, s.1);
        // Divide
        let mut i = bvh.first_object;
        let mut j = i + bvh.object_count - 1;
        while i <= j {
            if self.objects_bounds[i].centroid[divide_axis as usize] < split_pos {
                i += 1;
            }
            else {
                self.objects_bounds.swap(i, j);
                self.objects_indexes.swap(i, j);
                j -= 1;
            }
        }

        let left_count = i - bvh.first_object;
        // That's strange, if we divide in equal halfs it doesn't subdivide further
        if left_count == 0 || left_count == bvh.object_count {
            return;
        }
        // Set bvhs
        let left_node_index = self.nodes_used;
        self.nodes_used += 1;
        let right_node_index = self.nodes_used;
        self.nodes_used += 1;
        // Borrow checker wants this values to be copied
        // So we don't have 2 mutable references
        let first_object = bvh.first_object;
        let object_count = bvh.object_count;
        bvh.first_object = left_node_index;
        bvh.object_count = 0;
        
        self.bvhs[left_node_index].first_object = first_object;
        self.bvhs[left_node_index].object_count = left_count;
        self.calculate_bvh_bounds(left_node_index);
        
        self.bvhs[right_node_index].first_object = i;
        self.bvhs[right_node_index].object_count = object_count - left_count;
        self.calculate_bvh_bounds(right_node_index);
    }

    #[inline]
    fn calculate_bvh_bounds(&mut self, bvh_index: usize) {
        let bvh = &mut self.bvhs[bvh_index];
        if self.objects_bounds.is_empty() || bvh.object_count == 0 {
            return;
        }
        let triangles: &[Bounds] = &self.objects_bounds[(bvh.first_object)..(bvh.first_object + bvh.object_count)];
        let points: Vec<Vector3<f32>> = triangles.iter().flat_map(|x| [x.aabb_min, x.aabb_max]).collect();

        let x_min = points.iter().map(|x: &Vector3<f32>| x.x).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let y_min = points.iter().map(|x: &Vector3<f32>| x.y).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let z_min = points.iter().map(|x: &Vector3<f32>| x.z).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();

        let x_max = points.iter().map(|x: &Vector3<f32>| x.x).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let y_max = points.iter().map(|x: &Vector3<f32>| x.y).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let z_max = points.iter().map(|x: &Vector3<f32>| x.z).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();

        bvh.aabb_min = Vector3::new(x_min, y_min, z_min);
        bvh.aabb_max = Vector3::new(x_max, y_max, z_max);
    }

    #[inline]
    fn calculate_bvh_centroids(&self, bvh_index: usize) -> Option<(Vector3<f32>, Vector3<f32>)> {
        let bvh = &self.bvhs[bvh_index];
        if self.objects_bounds.is_empty() || bvh.object_count == 0 {
            return None;
        }
        let triangles: &[Bounds] = &self.objects_bounds[(bvh.first_object)..(bvh.first_object + bvh.object_count)];
        let points: Vec<Vector3<f32>> = triangles.iter().map(|x| x.centroid).collect();

        let x_min = points.iter().map(|x: &Vector3<f32>| x.x).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let y_min = points.iter().map(|x: &Vector3<f32>| x.y).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let z_min = points.iter().map(|x: &Vector3<f32>| x.z).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();

        let x_max = points.iter().map(|x: &Vector3<f32>| x.x).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let y_max = points.iter().map(|x: &Vector3<f32>| x.y).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let z_max = points.iter().map(|x: &Vector3<f32>| x.z).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();

        Some((Vector3::new(x_min, y_min, z_min), Vector3::new(x_max, y_max, z_max)))
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::Vector3;
    use crate::math::ray::Ray;
    use super::BvhNode;

    #[test]
    fn bvh_intersection() {
        let mut bvh = BvhNode::new(0, 0);
        bvh.aabb_min = Vector3::new(-1.0, -1.0, -1.0);
        bvh.aabb_max = Vector3::new(1.0, 1.0, 1.0);

        let ray = Ray::new(
            Vector3::new(0.0, 0.0, -5.0),
            Vector3::new(0.0, 0.0, 1.0)
        );

        let result = bvh.intersect(&ray);
        assert!(result);
    }
    
    #[test]
    fn division_plane() {
        let mut bvh = BvhNode::new(0, 0);
        bvh.aabb_min = Vector3::new(-1.0, -1.0, -2.0);
        bvh.aabb_max = Vector3::new(1.0, 1.0, 2.0);

        //let (split_pos, division_plane) = bvh.division_plane();
        //assert_eq!(division_plane, 2);
        //assert_eq!(split_pos, 0.0);
    }
}