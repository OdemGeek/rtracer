use std::sync::Arc;
use nalgebra::Vector3;
use crate::{math::{ray::Ray, pcg}, material::Material, entity::hit::Intersection};
use super::hit::Hittable;

// Maybe change it to pointer to vertex slice of vertexes
#[derive(Debug)]
pub struct Triangle {
    vertex1: Vector3<f32>,
    vertex2: Vector3<f32>,
    vertex3: Vector3<f32>,
    normal: Vector3<f32>,
    pub material: Arc<Material>,
}

#[allow(dead_code)]
impl Triangle {
    #[inline]
    pub fn new(vertex1: Vector3<f32>, vertex2: Vector3<f32>, vertex3: Vector3<f32>, material: Arc<Material>) -> Self {
        let mut x = Triangle {
            vertex1, vertex2, vertex3, material,
            normal: Vector3::zeros(),
        };
        x.normal = x.plane_normal();
        x
    }

    #[inline(always)]
    pub fn vertex1(&self) -> Vector3<f32> {
        self.vertex1
    }

    #[inline(always)]
    pub fn vertex2(&self) -> Vector3<f32> {
        self.vertex2
    }

    #[inline(always)]
    pub fn vertex3(&self) -> Vector3<f32> {
        self.vertex3
    }

    #[inline]
    pub fn random_point(&self, seed: &mut u32) -> Vector3<f32> {
        // Shape Distributions
        // ROBERT OSADA, THOMAS FUNKHOUSER, BERNARD CHAZELLE, and DAVID DOBKIN
        // Princeton University
        // P = (1 - sqrt(r1))*A + sqrt(r1)*(1 - r2)*B + sqrt(r1)*r2*C
        // Where A, B, C is vertices and r1, r2 is uniform random values in range 0-1
        let r1sqrt = pcg::random_f32(seed).sqrt();
        let r2 = pcg::random_f32(seed);
        (1.0 - r1sqrt) * self.vertex1 + r1sqrt * (1.0 - r2) * self.vertex2 + r1sqrt * r2 * self.vertex3
    }

    // Code provided by ChatGPT
    #[inline]
    pub fn plane_normal(&self) -> Vector3<f32> {
        // Calculate the normal vector of the triangle (cross product of two sides)
        let v1 = Vector3::new(
            self.vertex2.x - self.vertex1.x,
            self.vertex2.y - self.vertex1.y,
            self.vertex2.z - self.vertex1.z,
        );
        let v2 = Vector3::new(
            self.vertex3.x - self.vertex1.x,
            self.vertex3.y - self.vertex1.y,
            self.vertex3.z - self.vertex1.z,
        );

        Vector3::new(
            v1.y * v2.z - v1.z * v2.y,
            v1.z * v2.x - v1.x * v2.z,
            v1.x * v2.y - v1.y * v2.x,
        ).normalize()
    }

    #[inline(always)]
    pub fn normal_flipped(&self, ray_direction: &Vector3<f32>) -> Vector3<f32> {
        let direction = self.normal.dot(ray_direction);
        let is_flipped = if direction > 0.0 {-1.0} else {1.0};
        self.normal * is_flipped
    }
}

#[allow(dead_code)]
impl Hittable<Self> for Triangle {
    // Möller–Trumbore intersection algorithm, but some lines changed
    #[inline]
    #[allow(clippy::manual_range_contains)]
    fn intersect(&self, ray: &Ray) -> Option<Intersection<Self>> {
        const EPSILON: f32 = 0.0000001;
        let edge1 = self.vertex2 - self.vertex1;
        let edge2 = self.vertex3 - self.vertex1;
        let h = ray.direction.cross(&edge2);
        let a = edge1.dot(&h);
        
        // Without this check render is faster
        // if a > -EPSILON && a < EPSILON {
        //     return None; // This ray is parallel to this triangle.
        // } 

        let f = 1.0 / a;
        let s = ray.origin - self.vertex1;
        let u = f * s.dot(&h);
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(&edge1);
        // At this stage we can compute t to find out where the intersection point is on the line.
        let t = f * edge2.dot(&q);
        // Added `t` check early and moved `t` calculation up by myself, in tests it's faster
        if t <= EPSILON {
            return None;
        }

        let v = f * ray.direction.dot(&q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // Somehow code is becoming faster when removing second `t` check
        if t > EPSILON // ray intersection
        {
            Some(Intersection::new(t, self))
        }
        else { // This means that there is a line intersection but not a ray intersection.
            None
        }
        
    }

    #[inline(always)]
    fn normal(&self, ray_direction: &Vector3<f32>) -> Vector3<f32> {
        self.normal_flipped(ray_direction)
    }
}
