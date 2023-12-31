mod bvh_intersection;
mod bvh_depth;
mod bvh_node;
use bvh_intersection::BvhIntersection;
pub use bvh_depth::BvhDepth;
pub use bvh_node::BvhNode;

use nalgebra::Vector3;
use crate::math::ray::Ray;
use crate::entity::hit::{Hit, Hittable};
use crate::entity::Bounds;

pub struct Bvh {
    bvhs: Vec<BvhNode>,
    objects_bounds: Vec<Bounds>,
    objects_centroids: Vec<Vector3<f32>>,
    objects_indexes: Vec<usize>,
    nodes_used: usize
}

impl Default for Bvh {
    #[inline]
    fn default() -> Self {
        Bvh {
            bvhs: vec![],
            objects_bounds: vec![],
            objects_centroids: vec![],
            objects_indexes: vec![],
            nodes_used: 0
        }
    }
}

impl Bvh {
    #[inline]
    pub fn intersect<'a, T>(&'a self, ray: &Ray, objects: &'a [T]) -> Option<Hit<T>>
    where T: Hittable<T> {
        let ray = ray.clone();
        // Get closest hit
        let mut bvh_intersection = BvhIntersection::new(self, &ray, objects);
        bvh_intersection.intersect_hierarchy();
        bvh_intersection.closest_hit.map(|hit| Hit::<'a, T> {
            t: hit.t,
            point: ray.origin + ray.get_direction() * hit.t,
            object: hit.object
        })
    }

    #[inline]
    pub fn bvh_count(&self) -> usize {
        self.bvhs.len()
    }

    #[inline]
    pub fn get_bvh_by_depth(&self, depth: u32) -> Vec<BvhNode> {
        let mut bd = BvhDepth::new(&self.bvhs, depth);
        bd.intersect_hierarchy();
        bd.bvhs
    }

    #[inline]
    pub fn calculate_bvh(&mut self, objects_bounds: Vec<Bounds>, objects_centroids: Vec<Vector3<f32>>) {
        self.objects_indexes = (0..objects_bounds.len()).collect();
        self.objects_bounds = objects_bounds;
        self.objects_centroids = objects_centroids;

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
    }

    #[inline]
    fn calculate_childs(&mut self, bvh_index: usize) {
        let bvh = &self.bvhs[bvh_index];

        let e: Vector3<f32> = bvh.aabb_max - bvh.aabb_min; // extent of parent
        let node_area: f32 = e.x * e.y + e.y * e.z + e.z * e.x;
        let node_cost: f32 = bvh.object_count as f32 * node_area;

        let (split_pos, divide_axis, best_cost) = self.division_plane(bvh);
        if best_cost >= node_cost {
            return;
        }
        // Divide
        let mut i = bvh.first_object;
        let mut j = i + bvh.object_count - 1;
        while i <= j {
            if self.objects_centroids[i][divide_axis as usize] < split_pos {
                i += 1;
            }
            else {
                self.objects_bounds.swap(i, j);
                self.objects_indexes.swap(i, j);
                self.objects_centroids.swap(i, j);
                j -= 1;
            }
        }
        let left_count = i - bvh.first_object;
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
        let bvh = &mut self.bvhs[bvh_index];
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
    /// Returns (split_pos, divide_axis, best_cost)
    pub fn division_plane(&self, bvh: &BvhNode) -> (f32, i32, f32) {
        // determine split axis using SAH
        let mut best_axis: usize = 0;
        let mut best_pos: f32 = 0.0;
        let mut best_cost: f32 = 1e30;
        for axis in 0..3 {
            
            // Calculate bounds by centroids of objects in this node
            let mut bounds_min: f32 = 1e30;
            let mut bounds_max: f32 = -1e30;
            for i in 0..bvh.object_count {
                let centroid = &self.objects_centroids[bvh.first_object + i];
                bounds_min = bounds_min.min(centroid[axis]);
                bounds_max = bounds_max.max(centroid[axis]);
            }
            if bounds_min == bounds_max {
                continue;
            }
            let scale: f32 = (bounds_max - bounds_min) / 16.0;
            for i in 0..16 {
                let candidate_pos: f32 = bounds_min + i as f32 * scale;
                let cost: f32 = self.evaluate_sah(bvh, axis, candidate_pos);
                if cost < best_cost {
                    best_axis = axis;
                    best_pos = candidate_pos;
                    best_cost = cost;
                }
            }
            // Bins implementation (don't work)
            /*
            const BINS: usize = 8;
            // Calculate bounds by centroids of objects in this node
            let mut bounds_min: f32 = 1e30;
            let mut bounds_max: f32 = -1e30;
            for i in 0..bvh.object_count {
                let centroid = &self.objects_centroids[bvh.first_object + i];
                bounds_min = bounds_min.min(centroid[axis]);
                bounds_max = bounds_max.max(centroid[axis]);
            }
            if bounds_min == bounds_max {
                continue;
            }
            // populate the bins
            let mut bins: [Bin; BINS] = [Bin::default(); BINS];
            let mut scale: f32 = BINS as f32 / (bounds_max - bounds_min);
            for i in 0..bvh.object_count {
                let centroid = &self.objects_centroids[bvh.first_object + i];
                let bounds = &self.objects_bounds[bvh.first_object + i];
                let bin_ind: usize = usize::min(BINS - 1, ((centroid[axis] - bounds_min) * scale) as usize);
                bins[bin_ind].object_count += 1;
                bins[bin_ind].aabb.grow(bounds.aabb_min);
                bins[bin_ind].aabb.grow(bounds.aabb_max);
            }
            // gather data for the (BINS count - 1) planes between the (BINS count) bins
            let mut left_area: [f32; BINS - 1] = [0.0; BINS - 1];
            let mut right_area: [f32; BINS - 1] = [0.0; BINS - 1];
            let mut left_count: [usize; BINS - 1] = [0; BINS - 1];
            let mut right_count: [usize; BINS - 1] = [0; BINS - 1];
            let mut left_box: Aabb = Aabb::default();
            let mut right_box: Aabb = Aabb::default();
            let mut left_sum = 0;
            let mut right_sum = 0;

            for i in 0..BINS - 1 {
                left_sum += bins[i].object_count;
                left_count[i] = left_sum;
                left_box.grow(bins[i].aabb.bmin);
                left_box.grow(bins[i].aabb.bmax);
                left_area[i] = left_box.area();

                right_sum += bins[BINS - 1 - i].object_count;
                right_count[BINS - 2 - i] = right_sum;
                right_box.grow(bins[BINS - 1 - i].aabb.bmin);
                right_box.grow(bins[BINS - 1 - i].aabb.bmax);
                right_area[BINS - 2 - i] = right_box.area();
            }
            // calculate SAH cost for the (BINS count) planes
            scale = (bounds_max - bounds_min) / BINS as f32;
            for i in 0..BINS - 1 {
                let plane_cost: f32 = left_count[i] as f32 * left_area[i] + right_count[i] as f32 * right_area[i];
                if plane_cost < best_cost {
                    best_axis = axis;
                    best_pos = bounds_min + scale * (i + 1) as f32;
                    best_cost = plane_cost;
                }
            }*/
        }
        let axis: i32 = best_axis as i32;
        let split_pos: f32 = best_pos;
        (split_pos, axis, best_cost)
    }

    #[inline]
    fn evaluate_sah(&self, bvh: &BvhNode, axis: usize, pos: f32) -> f32 {
        // determine triangle counts and bounds for this split candidate
        let mut left_box = Aabb::default();
        let mut right_box = Aabb::default();
        let mut left_count = 0;
        let mut right_count = 0;
        for i in 0..bvh.object_count
        {
            let centroid = &self.objects_centroids[bvh.first_object + i];
            let bounds = &self.objects_bounds[bvh.first_object + i];
            if centroid[axis] < pos {
                left_count += 1;
                left_box.grow(bounds.aabb_min);
                left_box.grow(bounds.aabb_max);
            } else {
                right_count += 1;
                right_box.grow(bounds.aabb_min);
                right_box.grow(bounds.aabb_max);
            }
        }
        let cost: f32 = left_count as f32 * left_box.area() + right_count as f32 * right_box.area();
        if cost > 0.0 { cost } else { 1e30 }
    }

    #[inline]
    fn calculate_bvh_bounds(&mut self, bvh_index: usize) {
        let bvh = &mut self.bvhs[bvh_index];
        if self.objects_bounds.is_empty() || bvh.object_count == 0 {
            return;
        }
        let objects: &[Bounds] = &self.objects_bounds[(bvh.first_object)..(bvh.first_object + bvh.object_count)];
        let points: Vec<Vector3<f32>> = objects.iter().flat_map(|x| [x.aabb_min, x.aabb_max]).collect();

        let mut min: Vector3<f32> = Vector3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max: Vector3<f32> = Vector3::new(-f32::INFINITY, -f32::INFINITY, -f32::INFINITY);
        points.iter().for_each(|x: &Vector3<f32>| {
            min.x = min.x.min(x.x);
            min.y = min.y.min(x.y);
            min.z = min.z.min(x.z);

            max.x = max.x.max(x.x);
            max.y = max.y.max(x.y);
            max.z = max.z.max(x.z);
        });

        bvh.aabb_min = min;
        bvh.aabb_max = max;
    }
}

/*#[derive(Default, Clone)]
struct Bin {
    pub aabb: Aabb,
    pub object_count: usize
}*/

#[derive(Clone)]
struct Aabb {
    pub bmin: Vector3<f32>,
    pub bmax: Vector3<f32>
}
impl Default for Aabb {
    fn default() -> Self {
        Self { bmin: Vector3::new(1e30, 1e30, 1e30), bmax: Vector3::new(-1e30, -1e30, -1e30) }
    }
}
impl Aabb {
    fn grow(&mut self, p: Vector3<f32>) {
        self.bmin = Vector3::new(self.bmin.x.min(p.x), self.bmin.y.min(p.y), self.bmin.z.min(p.z));
        self.bmax = Vector3::new(self.bmax.x.max(p.x), self.bmax.y.max(p.y), self.bmax.z.max(p.z));
    }

    fn area(&self) -> f32 { 
        let e = self.bmax - self.bmin; // box extent
        e.x * e.y + e.y * e.z + e.z * e.x
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::Vector3;
    use crate::math::ray::Ray;
    use super::{BvhNode, Aabb};

    #[test]
    fn aabb_grow() {
        let mut aabb = Aabb::default();
        aabb.grow(Vector3::new(-1.0, 0.0, 0.5));
        aabb.grow(Vector3::new(1.0, 0.5, 2.0));
        aabb.grow(Vector3::new(0.01, 0.01, 0.7));
        aabb.grow(Vector3::new(0.1, 0.2, 0.8));
        assert_eq!(aabb.bmin, Vector3::new(-1.0, 0.0, 0.5));
        assert_eq!(aabb.bmax, Vector3::new(1.0, 0.5, 2.0));
    }
    
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