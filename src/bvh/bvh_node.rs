use nalgebra::Vector3;
use crate::{math::ray::Ray, entity::{hit::{Intersection, Hittable}, Bounds}};

#[derive(Debug, Clone, Default)]
pub struct BvhNode {
    pub aabb_min: Vector3<f32>,
    pub aabb_max: Vector3<f32>,
    /// Also used as left_node_index
    pub first_object: usize,
    pub object_count: usize,
}

impl BvhNode {
    #[inline]
    pub fn new(first_object: usize, object_count: usize) -> Self {
        BvhNode {
            aabb_min: Vector3::zeros(),
            aabb_max: Vector3::zeros(),
            first_object,
            object_count
        }
    }

    #[inline]
    pub fn intersect(&self, ray: &Ray) -> bool {
        let dirfrac = ray.get_frac_direction();
        
        let tx1 = (self.aabb_min.x - ray.origin.x) * dirfrac.x;
        let tx2 = (self.aabb_max.x - ray.origin.x) * dirfrac.x;
        let mut tmin = tx1.min(tx2);
        let mut tmax = tx1.max(tx2);
        let ty1 = (self.aabb_min.y - ray.origin.y) * dirfrac.y;
        let ty2 = (self.aabb_max.y - ray.origin.y) * dirfrac.y;
        tmin = tmin.max(ty1.min(ty2));
        tmax = tmax.min(ty1.max(ty2));
        let tz1 = (self.aabb_min.z - ray.origin.z) * dirfrac.z;
        let tz2 = (self.aabb_max.z - ray.origin.z) * dirfrac.z;
        tmin = tmin.max(tz1.min(tz2));
        tmax = tmax.min(tz1.max(tz2));
        tmax >= tmin && tmax > 0.0
    }

    #[inline]
    pub fn intersect_point(&self, ray: &Ray) -> Option<Intersection<BvhNode>> {
        let dirfrac = ray.get_frac_direction();

        let t1 = (self.aabb_min.x - ray.origin.x) * dirfrac.x;
        let t2 = (self.aabb_max.x - ray.origin.x) * dirfrac.x;
        let t3 = (self.aabb_min.y - ray.origin.y) * dirfrac.y;
        let t4 = (self.aabb_max.y - ray.origin.y) * dirfrac.y;
        let t5 = (self.aabb_min.z - ray.origin.z) * dirfrac.z;
        let t6 = (self.aabb_max.z - ray.origin.z) * dirfrac.z;

        let tmin = f32::max(f32::max(f32::min(t1, t2),
            f32::min(t3, t4)), f32::min(t5, t6));
        let tmax = f32::min(f32::min(f32::max(t1, t2),
            f32::max(t3, t4)), f32::max(t5, t6));

        // if tmax < 0, ray (line) is intersecting AABB, but the whole AABB is behind us
        if tmax < 0.0 {
            // t = tmax;
            return None;
        }

        // if tmin > tmax, ray doesn't intersect AABB
        if tmin > tmax {
            // t = tmax;
            return None;
        }

        if tmin < 0.0 {
            return Some(Intersection::<BvhNode>::new(tmax, self));
        }

        if tmin < 0.0 {
            return Some(Intersection::<BvhNode>::new(tmax, self));
        }

        // t = tmin;
        Some(Intersection::<BvhNode>::new(tmin, self))
    }

    #[inline]
    pub fn intersect_distance(&self, ray: &Ray) -> f32 {
        let dirfrac = ray.get_frac_direction();

        let t1 = (self.aabb_min.x - ray.origin.x) * dirfrac.x;
        let t2 = (self.aabb_max.x - ray.origin.x) * dirfrac.x;
        let t3 = (self.aabb_min.y - ray.origin.y) * dirfrac.y;
        let t4 = (self.aabb_max.y - ray.origin.y) * dirfrac.y;
        let t5 = (self.aabb_min.z - ray.origin.z) * dirfrac.z;
        let t6 = (self.aabb_max.z - ray.origin.z) * dirfrac.z;

        let tmin = f32::max(f32::max(f32::min(t1, t2),
            f32::min(t3, t4)), f32::min(t5, t6));
        let tmax = f32::min(f32::min(f32::max(t1, t2),
            f32::max(t3, t4)), f32::max(t5, t6));
        
        if tmax >= tmin && tmax > 0.0 {
            tmin
        } else {
            f32::INFINITY
        }
        // if tmax < 0, ray (line) is intersecting AABB, but the whole AABB is behind us
        /*if tmax < 0.0 {
            // t = tmax;
            return None;
        }

        // if tmin > tmax, ray doesn't intersect AABB
        if tmin > tmax {
            // t = tmax;
            return None;
        }

        if tmin < 0.0 {
            return Some(tmax);
        }

        if tmin < 0.0 {
            return Some(tmax);
        }

        // t = tmin;
        Some(tmin)*/
    }


    #[inline]
    pub fn distance_to_edge(&self, point: &Vector3<f32>) -> f32 {
        let ma_x = 
            (if (point.x - self.aabb_min.x).abs() < 0.00001 {std::f32::MAX} else {point.x - self.aabb_min.x}).abs()
            .min((if (point.x - self.aabb_max.x).abs() < 0.00001 {std::f32::MAX} else {point.x - self.aabb_max.x}).abs());
        let ma_y = 
            (if (point.y - self.aabb_min.y).abs() < 0.00001 {std::f32::MAX} else {point.y - self.aabb_min.y}).abs()
            .min((if (point.y - self.aabb_max.y).abs() < 0.00001 {std::f32::MAX} else {point.y - self.aabb_max.y}).abs());
        let ma_z = 
            (if (point.z - self.aabb_min.z).abs() < 0.00001 {std::f32::MAX} else {point.z - self.aabb_min.z}).abs()
            .min((if (point.z - self.aabb_max.z).abs() < 0.00001 {std::f32::MAX} else {point.z - self.aabb_max.z}).abs());
        ma_x.min(ma_y).min(ma_z)
    }

    #[inline(always)]
    pub fn is_leaf(&self) -> bool {
        self.object_count > 0
    }
}

impl From<BvhNode> for Bounds {
    #[inline]
    fn from(value: BvhNode) -> Self {
        Bounds {
            centroid: (value.aabb_max + value.aabb_min) / 2.0,
            aabb_min: value.aabb_min,
            aabb_max: value.aabb_max
        }
    }
}

impl From<&BvhNode> for Bounds {
    #[inline]
    fn from(value: &BvhNode) -> Self {
        Bounds {
            centroid: (value.aabb_max + value.aabb_min) / 2.0,
            aabb_min: value.aabb_min,
            aabb_max: value.aabb_max
        }
    }
}

impl Hittable<BvhNode> for BvhNode {
    #[inline]
    fn intersect(&self, ray: &Ray) -> Option<Intersection<BvhNode>> {
        self.intersect_point(ray)
    }

    #[inline]
    fn normal(&self, ray_direction: &Vector3<f32>) -> Vector3<f32> {
        *ray_direction
    }
}