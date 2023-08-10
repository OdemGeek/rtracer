use nalgebra::Vector3;
use crate::math::ray::Ray;
use super::{triangle::Triangle, hit::Intersection};

#[derive(Debug)]
pub struct Bvh {
    pub aabb_min: Vector3<f32>,
    pub aabb_max: Vector3<f32>,
    pub left_node_index: Option<u32>,
    pub right_node_index: Option<u32>,
    objects_indexes: Vec<u32>
}

impl Bvh {
    #[inline]
    pub fn new(objects: Vec<u32>) -> Self {
        Bvh { aabb_min: Vector3::zeros(), aabb_max: Vector3::zeros(), left_node_index: None, right_node_index: None, objects_indexes: objects }
    }

    #[inline]
    pub fn set_objects(&mut self, objects: Vec<u32>) {
        self.objects_indexes = objects;
    }

    #[inline]
    pub fn calculate_children(&mut self, objects: &[Triangle]) -> Option<(Bvh, Bvh)> {
        // Calculate middle plane
        let x_len = (self.aabb_max.x - self.aabb_min.x).abs();
        let y_len = (self.aabb_max.y - self.aabb_min.y).abs();
        let z_len = (self.aabb_max.z - self.aabb_min.z).abs();

        let direction: Vector3<f32> = if x_len >= y_len && x_len >= z_len {
            Vector3::x()
        } else if y_len >= x_len && y_len >= z_len {
            Vector3::y()
        } else {
            Vector3::z()
        };
        let middle_plane = Ray::new((self.aabb_max + self.aabb_min) / 2.0, direction);
        
        // Our triangles
        let triangles: Vec<&Triangle> = self.objects_indexes.iter().map(|x| &objects[*x as usize]).collect();

        // Do we need division?

        let mut left_objects: Vec<u32> = vec![];
        let mut right_objects: Vec<u32> = vec![];
        // Divide
        for tr in triangles.iter().enumerate() {
            let relative_position = Self::plane_relative_position(&middle_plane, tr.1);
            if relative_position.0 {
                left_objects.push(tr.0 as u32);
            }
            if relative_position.1 {
                right_objects.push(tr.0 as u32);
            }
        }
        if left_objects.is_empty() || right_objects.is_empty() {
            return None;
        }
        // Set bvhs
        let mut left_bvh = Bvh::new(left_objects);
        let mut right_bvh = Bvh::new(right_objects);
        left_bvh.calculate_bounds(objects);
        right_bvh.calculate_bounds(objects);
        
        Some((left_bvh, right_bvh))
    }

    // Distance to plane from point = n * (a - p)
    // Where n - plane normal, p - plane pos, a - point pos
    // Do this for all three points to get info about the triangle for an example
    /// (is in front, is behind)
    #[inline]
    fn plane_relative_position(plane: &Ray, triangle: &Triangle) -> (bool, bool) {
        let vertex1 = plane.direction.dot(&(triangle.vertex1() - plane.origin));
        let vertex2 = plane.direction.dot(&(triangle.vertex2() - plane.origin));
        let vertex3 = plane.direction.dot(&(triangle.vertex3() - plane.origin));

        let (mut in_front, mut behind) = (false, false);
        if vertex1 > 0.0 || vertex2 > 0.0 || vertex3 > 0.0 {
            in_front = true;
        }
        else if vertex1 < 0.0 || vertex2 < 0.0 || vertex3 < 0.0 {
            behind = true;
        }
        (in_front, behind)
    }

    #[inline]
    pub fn calculate_bounds(&mut self, objects: &[Triangle]) {
        if objects.is_empty() || self.objects_indexes.is_empty() {
            return;
        }
        let triangles: Vec<&Triangle> = self.objects_indexes.iter().map(|x| &objects[*x as usize]).collect();
        let points: Vec<Vector3<f32>> = triangles.into_iter().flat_map(|x| [x.vertex1(), x.vertex2(), x.vertex3()]).collect();

        let x_min = points.iter().map(|x: &Vector3<f32>| x.x).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let y_min = points.iter().map(|x: &Vector3<f32>| x.y).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let z_min = points.iter().map(|x: &Vector3<f32>| x.z).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();

        let x_max = points.iter().map(|x: &Vector3<f32>| x.x).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let y_max = points.iter().map(|x: &Vector3<f32>| x.y).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let z_max = points.iter().map(|x: &Vector3<f32>| x.z).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();

        self.aabb_min = Vector3::new(x_min, y_min, z_min);
        self.aabb_max = Vector3::new(x_max, y_max, z_max);
    }

    #[inline]
    pub fn intersect(&self, ray: &Ray) -> Option<Intersection<Self>> {
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

        // t = tmin;
        Some(Intersection::new(tmin, self))
    }
}

pub struct BoundsTriangle {
    pub object_index: u32,
    pub centroid: Vector3<f32>,
    pub aabb_min: Vector3<f32>,
    pub aabb_max: Vector3<f32>,
}

impl BoundsTriangle {
    pub fn new(object_index: u32, point1: Vector3<f32>, point2: Vector3<f32>, point3: Vector3<f32>) -> Self {
        let mut x = BoundsTriangle { object_index, centroid: (point1 + point2 + point3) / 3.0, aabb_min: Vector3::zeros(), aabb_max: Vector3::zeros() };
        x.bounds_from_points(point1, point2, point3);
        x
    }

    pub fn bounds_from_points(&mut self, point1: Vector3<f32>, point2: Vector3<f32>, point3: Vector3<f32>) {
        let points: Vec<Vector3<f32>> = vec![point1, point2, point3];

        let x_min = points.iter().map(|x: &Vector3<f32>| x.x).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let y_min = points.iter().map(|x: &Vector3<f32>| x.y).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let z_min = points.iter().map(|x: &Vector3<f32>| x.z).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();

        let x_max = points.iter().map(|x: &Vector3<f32>| x.x).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let y_max = points.iter().map(|x: &Vector3<f32>| x.y).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let z_max = points.iter().map(|x: &Vector3<f32>| x.z).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();

        self.aabb_min = Vector3::new(x_min, y_min, z_min);
        self.aabb_max = Vector3::new(x_max, y_max, z_max);
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::Vector3;
    use crate::entity::bvh::BoundsTriangle;

    #[test]
    fn triangle_bounds_from_points() {
        let triangle = BoundsTriangle::new(
            0,
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0)
        );
        assert_eq!(triangle.aabb_min, Vector3::new(0.0, 0.0, 0.0));
        assert_eq!(triangle.aabb_max, Vector3::new(1.0, 1.0, 0.0));
    }
}