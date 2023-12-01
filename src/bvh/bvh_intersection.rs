use crate::{entity::hit::{Hittable, Intersection}, math::ray::Ray};
use super::{Bvh, BvhNode};

pub(super) struct BvhIntersection<'a, 'b, T> {
    pub closest_hit: Option<Intersection<'b, T>>,
    closest_dist: f32,
    data: &'a Bvh,
    objects: &'b [T],
    ray: &'a Ray
}

impl<'a, 'b, T>  BvhIntersection<'a, 'b, T>
where T: Hittable<T> {
    #[inline]
    pub fn new(data: &'a Bvh, ray: &'a Ray, objects: &'b [T]) -> BvhIntersection<'a, 'b, T> {
        BvhIntersection { data, closest_hit: None, closest_dist: f32::INFINITY, objects, ray }
    }

    #[inline]
    pub fn intersect_hierarchy(&mut self) {
        self.intersect_bvh(&self.data.bvhs[0]);

        // Non recursive code (much slower for now)
        /*let mut node: &BvhNode = &self.data.bvhs[0];
        let binding = BvhNode::default();
        let mut stack: [&BvhNode; 64] = [&binding; 64];
        let mut stack_ptr: usize = 0;
        loop {
            if node.is_leaf() {
                for i in 0..node.object_count {
                    self.intersect_triangles(node, ray, objects);
                }
                if stack_ptr == 0 {
                    break;
                } else { 
                    stack_ptr -= 1;
                    node = stack[stack_ptr];
                }
                continue;
            }
            let mut child1: &BvhNode = &self.data.bvhs[node.first_object];
            let mut child2: &BvhNode = &self.data.bvhs[node.first_object + 1];
            let mut dist1 = child1.intersect_distance(ray);
            let mut dist2 = child2.intersect_distance(ray);
            if dist1.unwrap_or(1e30) > dist2.unwrap_or(1e30) {
                (dist1, dist2) = (dist2, dist1);
                (child1, child2) = (child2, child1)
            }
            if dist1.is_some() {
                node = child1;
                if dist2.is_some() {
                    stack[stack_ptr] = child2;
                    stack_ptr += 1;
                }
            } else if stack_ptr == 0 {
                break;
            } else {
                stack_ptr -= 1;
                node = stack[stack_ptr];
            }
        }*/
    }

    #[inline]
    fn intersect_bvh(&mut self, node: &BvhNode) {
        if node.is_leaf() {
            self.intersect_triangles(node);
        } else {
            let child1 = &self.data.bvhs[node.first_object];
            let child2 = &self.data.bvhs[node.first_object + 1];
            let mut hit1 = child1.intersect_distance(self.ray);
            let mut hit2 = child2.intersect_distance(self.ray);
            let first: &BvhNode;
            let second: &BvhNode;
            if hit1 <= hit2 {
                first = child1;
                second = child2;
            } else {
                first = child2;
                second = child1;
                (hit1, hit2) = (hit2, hit1);
            };
            if hit1 < self.closest_dist {
                self.intersect_bvh(first);
            }
            if hit2 < self.closest_dist {
                self.intersect_bvh(second);
            }
        }
    }

    #[inline(always)]
    fn intersect_triangles(&mut self, bvh: &BvhNode) {
        let hit: Option<Intersection<'b, T>> = 
        self.data.objects_indexes[(bvh.first_object)..(bvh.first_object + bvh.object_count)]
        .iter().map(|x| {
            &self.objects[*x]
        })
        .filter_map(|obj| {  // Take valid hits
            obj.intersect(self.ray)
        })
        .min_by(|hit1, hit2| hit1.t.partial_cmp(&hit2.t).unwrap()); // Get min hit by param `t`

        if let Some(hit_u) = &hit {
            if let Some(closest_hit) = &self.closest_hit {
                if hit_u.t < closest_hit.t {
                    self.closest_dist = hit_u.t;
                    self.closest_hit = hit;
                }
            } else {
                self.closest_dist = hit_u.t;
                self.closest_hit = hit;
            }
        }
    }
}