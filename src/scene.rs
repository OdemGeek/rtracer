use std::time::Instant;
use nalgebra::Vector3;
use crate::{math::ray::Ray, entity::{hit::Hit, hit::{Hittable, Intersection}, triangle::Triangle, bvh::{Bvh, BoundsTriangle}}};

pub struct SceneData {
    pub objects: Vec<Triangle>,
    pub light_objects: Vec<u32>,
    bvhs: Vec<Bvh>,
    objects_bounds: Vec<BoundsTriangle>,
    root_node_idx: u32,
    nodes_used: u32,
}

impl SceneData {
    #[inline]
    pub fn new(objects: Vec<Triangle>) -> Self {
        let mut data = SceneData {
            objects,
            light_objects: vec![],
            bvhs: vec![],
            objects_bounds: vec![],
            root_node_idx: 0,
            nodes_used: 0
        };
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
        self.objects_bounds = Self::calculate_objects_bounds(&self.objects);
        println!("Bounds generation time: {} ms", timer.elapsed().as_millis());

        let timer = Instant::now();

        self.bvhs = Vec::with_capacity(self.objects.len() * 2 - 1);
        self.bvhs.resize(self.bvhs.capacity(), Bvh::new(0, 0));

        self.root_node_idx = 0;
        self.nodes_used = 1;

        let root_bvh = &mut self.bvhs[0];
        root_bvh.first_object = 0;
        root_bvh.object_count = self.objects.len() as u32;
        self.calculate_bounds(0);
        

        let mut current_index = 0;

        while current_index < self.nodes_used {
            self.calculate_childs(current_index);
            current_index += 1;
        }

        self.bvhs.resize(self.nodes_used as usize, Bvh::new(0, 0));

        println!("BVH generation time: {} ms", timer.elapsed().as_millis());
        self.bvhs.iter().enumerate().for_each(|x| {
            //if x.1.object_count > 0 {
                println!("{} {:?}\n", x.0, x.1);
            //}
        });
    }

    #[inline]
    pub fn calculate_childs(&mut self, bvh_index: u32) {
        let bvh = &mut self.bvhs[bvh_index as usize];
        let (split_pos, divide_axis) = bvh.division_plane();
        if bvh.object_count < 3 {
            return;
        }
        // Divide
        let mut i = bvh.first_object;
        let mut j = i + bvh.object_count - 1;
        while i <= j {
            if self.objects_bounds[i as usize].centroid[divide_axis as usize] < split_pos {
                i += 1;
            }
            else {
                j -= 1;
                self.objects_bounds.swap(i as usize, j as usize);
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
        bvh.first_object = left_node_index;
        
        self.bvhs[left_node_index as usize].first_object = first_object;
        self.bvhs[left_node_index as usize].object_count = left_count;
        self.calculate_bounds(left_node_index);
        self.bvhs[right_node_index as usize].first_object = i;
        self.bvhs[right_node_index as usize].object_count = object_count - left_count;
        self.calculate_bounds(right_node_index);
        self.bvhs[bvh_index as usize].object_count = 0;
    }

    #[inline]
    pub fn calculate_bounds(&mut self, bvh_index: u32) {
        let bvh = &mut self.bvhs[bvh_index as usize];
        if self.objects_bounds.is_empty() || bvh.object_count == 0 {
            return;
        }
        let triangles: &[BoundsTriangle] = &self.objects_bounds[(bvh.first_object as usize)..(bvh.first_object + bvh.object_count) as usize];
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
    fn calculate_objects_bounds(objects: &[Triangle]) -> Vec<BoundsTriangle> {
        objects.iter().enumerate().map(
            |x| BoundsTriangle::new(
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
            hit.object.normal_flipped(&ray.direction),
            hit.object
        ))
    }

    #[inline]
    pub fn get_bvh_by_depth(&self, depth: u32) -> Vec<&Bvh> {
        let mut bd = BvhDepth::new(self, depth);
        bd.intersect_hierarchy();
        bd.bvhs
    }
}

struct BvhDepth<'a> {
    pub data: &'a SceneData,
    pub depth: u32,
    pub bvhs: Vec<&'a Bvh>
}

impl BvhDepth<'_> {
    #[inline]
    pub fn new(data: &SceneData, depth: u32) -> BvhDepth<'_> {
        BvhDepth { data, depth, bvhs: vec![] }
    }

    #[inline]
    pub fn intersect_hierarchy(&mut self) {
        self.intersect_bvh(0, 0);
    }

    #[inline(always)]
    fn intersect_bvh(&mut self, bvh_index: u32, depth: u32) {
        let bvh = &self.data.bvhs[bvh_index as usize];

        if depth > self.depth {
            return;
        }
        if depth == self.depth {
            self.bvhs.push(bvh);
        } else if !bvh.is_leaf() {
            self.intersect_bvh(bvh.first_object, depth + 1 );
            self.intersect_bvh(bvh.first_object + 1, depth + 1);
        }
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

    #[inline(always)]
    fn intersect_bvh(&mut self, ray: &Ray, bvh_index: u32, ) {
        let bvh = &self.data.bvhs[bvh_index as usize];
        if !bvh.intersect(ray) {
            // return;
        }
        if bvh.is_leaf() {
            self.intersect_triangles(bvh, ray);
        }
        else {
            self.intersect_bvh(ray, bvh.first_object);
            self.intersect_bvh(ray, bvh.first_object + 1);
        }
    }

    #[inline(always)]
    fn intersect_triangles(&mut self, bvh: &Bvh, ray: &Ray) {
        let hit = self.data.objects[bvh.first_object as usize..(bvh.first_object + bvh.object_count) as usize].iter()
        .filter_map(|obj| {  // Take valid hits
            obj.intersect(ray)
        }) // Get min hit by param `t`
        .min_by(|hit1, hit2| hit1.t.partial_cmp(&hit2.t).unwrap());

        if let Some(ref closest_hit) = self.closest_hit {
            if let Some(ref hit_u) = hit {
                if hit_u.t < closest_hit.t {
                    self.closest_hit = hit;
                }
            }
        } else {
            self.closest_hit = hit;
        }
    }
}