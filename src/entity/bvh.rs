use nalgebra::Vector3;
use crate::math::ray::Ray;
use super::triangle::Triangle;

pub struct Bvh<'a> {
    aabb_min: Vector3<f32>,
    aabb_max: Vector3<f32>,
    left_node: Option<&'a Bvh<'a>>,
    right_node: Option<&'a Bvh<'a>>,
    objects_indexes: Vec<u32>
}

impl Bvh<'_> {
    #[inline]
    pub fn new(objects: Vec<u32>) -> Self {
        Bvh { aabb_min: Vector3::zeros(), aabb_max: Vector3::zeros(), left_node: None, right_node: None, objects_indexes: objects }
    }

    #[inline]
    pub fn set_objects(&mut self, objects: Vec<u32>) {
        self.objects_indexes = objects;
    }

    #[inline]
    pub fn calculate_bounds(&mut self, objects: &[Triangle]) {
        if objects.is_empty() || self.objects_indexes.is_empty() {
            return;
        }
        let triangles: Vec<&Triangle> = self.objects_indexes.iter().map(|x| &objects[*x as usize]).collect();
        let points: Vec<Vector3<f32>> = triangles.into_iter().flat_map(|x| [x.vertex1(), x.vertex2(), x.vertex3()]).collect();

        let x_min = points.iter().map(|x| x.x).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let y_min = points.iter().map(|x| x.y).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let z_min = points.iter().map(|x| x.z).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();

        let x_max = points.iter().map(|x| x.x).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let y_max = points.iter().map(|x| x.y).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let z_max = points.iter().map(|x| x.z).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();

        self.aabb_min = Vector3::new(x_min, y_min, z_min);
        self.aabb_max = Vector3::new(x_max, y_max, z_max);
    }

    #[inline]
    pub fn intersect(&self, ray: &Ray) -> bool {
        hit_aabb(&self.aabb_min, &self.aabb_max, ray)
    }
}

#[inline]
fn hit_aabb(aabb_min: &Vector3<f32>, aabb_max: &Vector3<f32>, ray: &Ray) -> bool {
	// r.dir is unit direction vector of ray
    let dirfrac = Vector3::new(
        1.0 / ray.direction.x,
        1.0 / ray.direction.y,
        1.0 / ray.direction.z
    );
    // lb is the corner of AABB with minimal coordinates - left bottom, rt is maximal corner
    // r.org is origin of ray
    let t1 = (aabb_min.x - ray.origin.x) * dirfrac.x;
    let t2 = (aabb_max.x - ray.origin.x) * dirfrac.x;
    let t3 = (aabb_min.y - ray.origin.y) * dirfrac.y;
    let t4 = (aabb_max.y - ray.origin.y) * dirfrac.y;
    let t5 = (aabb_min.z - ray.origin.z) * dirfrac.z;
    let t6 = (aabb_max.z - ray.origin.z) * dirfrac.z;

    let tmin = f32::max(f32::max(f32::min(t1, t2),
        f32::min(t3, t4)), f32::min(t5, t6));
    let tmax = f32::min(f32::min(f32::max(t1, t2),
        f32::max(t3, t4)), f32::max(t5, t6));

    // if tmax < 0, ray (line) is intersecting AABB, but the whole AABB is behind us
    if tmax < 0.0 {
        // t = tmax;
        return false;
    }

    // if tmin > tmax, ray doesn't intersect AABB
    if tmin > tmax {
        // t = tmax;
        return false;
    }

    // t = tmin;
    true
}	