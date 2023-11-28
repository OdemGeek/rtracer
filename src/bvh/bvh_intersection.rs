use crate::{entity::hit::{Hittable, Intersection}, math::ray::Ray};
use super::{Bvh, BvhNode};

pub(super) struct BvhIntersection<'a, T> {
    pub data: &'a Bvh,
    pub closest_hit: Option<Intersection<'a, T>>,
}

impl<'a, T>  BvhIntersection<'a, T>
where T: Hittable<T> {
    #[inline]
    pub fn new(data: &'a Bvh) -> BvhIntersection<'a, T> {
        BvhIntersection { data, closest_hit: None }
    }

    #[inline]
    pub fn intersect_hierarchy(&mut self, ray: &Ray, objects: &'a [T]) {
        self.intersect_bvh(ray, 0, objects);
    }

    #[inline]
    fn intersect_bvh(&mut self, ray: &Ray, bvh_index: usize, objects: &'a [T]) {
        let bvh = &self.data.bvhs[bvh_index];
        if !bvh.intersect(ray) {
            return;
        }
        if bvh.is_leaf() {
            self.intersect_triangles(bvh, ray, objects);
        } else {
            self.intersect_bvh(ray, bvh.first_object, objects);
            self.intersect_bvh(ray, bvh.first_object + 1, objects);
        }
    }

    #[inline(always)]
    fn intersect_triangles(&mut self, bvh: &BvhNode, ray: &Ray, objects: &'a [T]) {
        let hit = 
        self.data.objects_indexes[(bvh.first_object)..(bvh.first_object + bvh.object_count)]
        .iter().map(|x| {
            &objects[*x]
        })
        .filter_map(|obj| {  // Take valid hits
            obj.intersect(ray)
        })
        .min_by(|hit1, hit2| hit1.t.partial_cmp(&hit2.t).unwrap()); // Get min hit by param `t`

        if let Some(hit_u) = &hit {
            if let Some(closest_hit) = &self.closest_hit {
                if hit_u.t < closest_hit.t {
                    self.closest_hit = hit;
                }
            } else {
                self.closest_hit = hit;
            }
        }
    }
}