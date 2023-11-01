use nalgebra::Vector3;
use crate::math::ray::Ray;

use super::hit::Intersection;

#[derive(Debug, Clone)]
pub struct Bvh {
    pub aabb_min: Vector3<f32>,
    pub aabb_max: Vector3<f32>,
    /// Also used as left_node_index
    pub first_object: u32,
    pub object_count: u32,
}

impl Bvh {
    #[inline]
    pub fn new(first_object: u32, object_count: u32) -> Self {
        Bvh {
            aabb_min: Vector3::zeros(),
            aabb_max: Vector3::zeros(),
            first_object,
            object_count
        }
    }

    #[inline]
    pub fn intersect(&self, ray: &Ray) -> bool {
        let tx1 = (self.aabb_min.x - ray.origin.x) / ray.direction.x;
        let tx2 = (self.aabb_max.x - ray.origin.x) / ray.direction.x;
        let mut tmin = tx1.min(tx2);
        let mut tmax = tx1.max(tx2);
        let ty1 = (self.aabb_min.y - ray.origin.y) / ray.direction.y;
        let ty2 = (self.aabb_max.y - ray.origin.y) / ray.direction.y;
        tmin = tmin.max(ty1.min(ty2));
        tmax = tmax.min(ty1.max(ty2));
        let tz1 = (self.aabb_min.z - ray.origin.z) / ray.direction.z;
        let tz2 = (self.aabb_max.z - ray.origin.z) / ray.direction.z;
        tmin = tmin.max(tz1.min(tz2));
        tmax = tmax.min(tz1.max(tz2));
        tmax >= tmin && tmax > 0.0
    }

    #[inline]
    pub fn intersect_point<'a>(&'a self, ray: &Ray) -> Option<Vec<Intersection<'a, Bvh>>> {
        let dirfrac = Vector3::new(
            1.0 / ray.direction.x,
            1.0 / ray.direction.y,
            1.0 / ray.direction.z
        );

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
            return Some(vec![Intersection::<'a, Bvh>::new(tmax, self)]);
        }

        if tmin < 0.0 {
            return Some(vec![Intersection::<'a, Bvh>::new(tmax, self)]);
        }

        // t = tmin;
        Some(vec![Intersection::<'a, Bvh>::new(tmin, self), Intersection::<'a, Bvh>::new(tmax, self)])
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

    #[inline]
    pub fn division_plane(aabb_min: Vector3<f32>, aabb_max: Vector3<f32>) -> (f32, u32) {
        let x_len = aabb_max.x - aabb_min.x;
        let y_len = aabb_max.y - aabb_min.y;
        let z_len = aabb_max.z - aabb_min.z;

        if x_len >= y_len && x_len >= z_len {
            ((aabb_max.x + aabb_min.x) / 2.0, 0)
        } else if y_len >= x_len && y_len >= z_len {
            ((aabb_max.y + aabb_min.y) / 2.0, 1)
        } else {
            ((aabb_max.z + aabb_min.z) / 2.0, 2)
        }
    }
}

#[derive(Debug)]
pub struct BoundsTriangle {
    pub centroid: Vector3<f32>,
    pub aabb_min: Vector3<f32>,
    pub aabb_max: Vector3<f32>,
}

impl BoundsTriangle {
    #[inline]
    pub fn new(point1: Vector3<f32>, point2: Vector3<f32>, point3: Vector3<f32>) -> Self {
        let (min_bounds, max_bounds) = Self::bounds_from_points(point1, point2, point3);
        BoundsTriangle {
            centroid: (point1 + point2 + point3) / 3.0,
            aabb_min: min_bounds,
            aabb_max: max_bounds
        }
    }

    #[inline]
    fn bounds_from_points(point1: Vector3<f32>, point2: Vector3<f32>, point3: Vector3<f32>) -> (Vector3<f32>, Vector3<f32>) {
        let points: Vec<Vector3<f32>> = vec![point1, point2, point3];

        let x_min = points.iter().map(|x: &Vector3<f32>| x.x).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let y_min = points.iter().map(|x: &Vector3<f32>| x.y).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let z_min = points.iter().map(|x: &Vector3<f32>| x.z).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();

        let x_max = points.iter().map(|x: &Vector3<f32>| x.x).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let y_max = points.iter().map(|x: &Vector3<f32>| x.y).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let z_max = points.iter().map(|x: &Vector3<f32>| x.z).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();

        (Vector3::new(x_min, y_min, z_min), Vector3::new(x_max, y_max, z_max))
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::Vector3;
    use crate::{entity::bvh::BoundsTriangle, math::ray::Ray};
    use super::Bvh;

    #[test]
    fn triangle_bounds_from_points() {
        let triangle = BoundsTriangle::new(
            Vector3::new(-0.5, 0.0, 0.0),
            Vector3::new(1.2, 0.0, -0.25),
            Vector3::new(0.0, 1.0, 0.0)
        );
        assert_eq!(triangle.aabb_min, Vector3::new(-0.5, 0.0, -0.25));
        assert_eq!(triangle.aabb_max, Vector3::new(1.2, 1.0, 0.0));
    }

    #[test]
    fn bvh_intersection() {
        let mut bvh = Bvh::new(0, 0);
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
        let mut bvh = Bvh::new(0, 0);
        bvh.aabb_min = Vector3::new(-1.0, -1.0, -2.0);
        bvh.aabb_max = Vector3::new(1.0, 1.0, 2.0);

        //let (split_pos, division_plane) = bvh.division_plane();
        //assert_eq!(division_plane, 2);
        //assert_eq!(split_pos, 0.0);
    }

    
    #[test]
    fn plane_relative_position() {
        
    }
}