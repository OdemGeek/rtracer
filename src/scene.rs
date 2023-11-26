use std::time::Instant;
use nalgebra::Vector3;
use crate::{math::ray::Ray, entity::{hit::Hit, hit::Intersection, triangle::Triangle, bvh::{Bvh, BoundsTriangle}}};
use crate::entity::bvh_depth::BvhDepth;

pub struct SceneData {
    pub objects: Vec<Triangle>,
    bvhs: Vec<Bvh>,
    objects_bounds: Vec<BoundsTriangle>,
    nodes_used: usize,
}

impl SceneData {
    #[inline]
    pub fn new(objects: Vec<Triangle>) -> Self {
        SceneData {
            objects,
            bvhs: vec![],
            objects_bounds: vec![],
            nodes_used: 0
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
        let timer = Instant::now();
        self.objects_bounds = Self::calculate_objects_bounds(&self.objects);
        println!("Bounds generation time: {} ms", timer.elapsed().as_millis());

        let timer = Instant::now();

        self.bvhs = Vec::with_capacity(self.objects.len() * 2 - 1);
        self.bvhs.resize(self.bvhs.capacity(), Bvh::new(0, 0));

        self.nodes_used = 1;

        let root_bvh = &mut self.bvhs[0];
        root_bvh.first_object = 0;
        root_bvh.object_count = self.objects.len();
        self.calculate_bounds(0);
        

        let mut current_index = 0;

        while current_index < self.nodes_used {
            self.calculate_childs(current_index);
            current_index += 1;
        }

        self.bvhs.resize(self.nodes_used, Bvh::new(0, 0));

        println!("BVH generation time: {} ms", timer.elapsed().as_millis());
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
    pub fn calculate_childs(&mut self, bvh_index: usize) {
        let s: Option<(Vector3<f32>, Vector3<f32>)> = self.calculate_bounds_centroids(bvh_index);
        let bvh = &mut self.bvhs[bvh_index];
        if bvh.object_count < 3 {
            return;
        }
        let s: (Vector3<f32>, Vector3<f32>) = s.unwrap();
        let (split_pos, divide_axis) = Bvh::division_plane(s.0, s.1);
        // Divide
        let mut i = bvh.first_object;
        let mut j = i + bvh.object_count - 1;
        while i <= j {
            if self.objects_bounds[i].centroid[divide_axis as usize] < split_pos {
                i += 1;
            }
            else {
                self.objects_bounds.swap(i, j);
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
        self.calculate_bounds(left_node_index);
        
        self.bvhs[right_node_index].first_object = i;
        self.bvhs[right_node_index].object_count = object_count - left_count;
        self.calculate_bounds(right_node_index);
    }

    #[inline]
    pub fn calculate_bounds(&mut self, bvh_index: usize) {
        let bvh = &mut self.bvhs[bvh_index];
        if self.objects_bounds.is_empty() || bvh.object_count == 0 {
            return;
        }
        let triangles: &[BoundsTriangle] = &self.objects_bounds[(bvh.first_object)..(bvh.first_object + bvh.object_count)];
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
    pub fn calculate_bounds_centroids(&self, bvh_index: usize) -> Option<(Vector3<f32>, Vector3<f32>)> {
        let bvh = &self.bvhs[bvh_index];
        if self.objects_bounds.is_empty() || bvh.object_count == 0 {
            return None;
        }
        let triangles: &[BoundsTriangle] = &self.objects_bounds[(bvh.first_object)..(bvh.first_object + bvh.object_count)];
        let points: Vec<Vector3<f32>> = triangles.iter().map(|x| x.centroid).collect();

        let x_min = points.iter().map(|x: &Vector3<f32>| x.x).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let y_min = points.iter().map(|x: &Vector3<f32>| x.y).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let z_min = points.iter().map(|x: &Vector3<f32>| x.z).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();

        let x_max = points.iter().map(|x: &Vector3<f32>| x.x).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let y_max = points.iter().map(|x: &Vector3<f32>| x.y).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let z_max = points.iter().map(|x: &Vector3<f32>| x.z).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();

        Some((Vector3::new(x_min, y_min, z_min), Vector3::new(x_max, y_max, z_max)))
    }

    #[inline]
    fn calculate_objects_bounds(objects: &[Triangle]) -> Vec<BoundsTriangle> {
        objects.iter().enumerate().map(
            |x| BoundsTriangle::new(
                x.0,
                x.1.vertex1(),
                x.1.vertex2(),
                x.1.vertex3(),
            )
        ).collect()
    }

    #[inline]
    pub fn cast_ray(&self, ray: &Ray) -> Option<Hit> {
        // Get closest hit
        let mut bvh_intersection = BvhIntersection::new(self);
        bvh_intersection.intersect_hierarchy(ray);

        bvh_intersection.closest_hit.map(|hit| Hit::new(
            hit.t,
            ray.origin + ray.direction * hit.t,
            hit.object.normal(&ray.direction),
            hit.object
        ))
    }

    #[inline]
    pub fn get_bvh_by_depth(&self, depth: u32) -> Vec<&Bvh> {
        let mut bd = BvhDepth::new(&self.bvhs, depth);
        bd.intersect_hierarchy();
        bd.bvhs
    }
}

struct BvhIntersection<'a> {
    pub data: &'a SceneData,
    pub closest_hit: Option<Intersection<'a, Triangle>>,
}

impl BvhIntersection<'_> {
    #[inline]
    pub fn new(data: &SceneData) -> BvhIntersection<'_> {
        BvhIntersection { data, closest_hit: None }
    }

    #[inline]
    pub fn intersect_hierarchy(&mut self, ray: &Ray) {
        self.intersect_bvh(ray, 0);
    }

    #[inline]
    fn intersect_bvh(&mut self, ray: &Ray, bvh_index: usize) {
        let bvh = &self.data.bvhs[bvh_index];
        if !bvh.intersect(ray) {
            return;
        }
        if bvh.is_leaf() {
            self.intersect_triangles(bvh, ray);
        } else {
            self.intersect_bvh(ray, bvh.first_object);
            self.intersect_bvh(ray, bvh.first_object + 1);
        }
    }

    #[inline(always)]
    fn intersect_triangles(&mut self, bvh: &Bvh, ray: &Ray) {
        let hit = 
        self.data.objects_bounds[(bvh.first_object)..(bvh.first_object + bvh.object_count)]
        .iter().map(|x| {
            &self.data.objects[x.object_index]
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