use std::sync::Arc;
use nalgebra::{Vector3, Vector2};
use crate::{math::{ray::Ray, pcg}, material::Material, entity::hit::Intersection};

use super::{hit::Hittable, Bounds};

// Maybe change it to pointer to vertex slice of vertexes
#[derive(Debug)]
pub struct Triangle {
    vertex1: Vector3<f32>,
    vertex2: Vector3<f32>,
    vertex3: Vector3<f32>,
    norm1: Vector3<f32>,
    norm2: Vector3<f32>,
    norm3: Vector3<f32>,
    normal: Vector3<f32>,
    uv1: Vector2<f32>,
    uv2: Vector2<f32>,
    uv3: Vector2<f32>,
    pub material: Arc<Material>,
    pub index: usize,
}

#[allow(dead_code)]
impl Triangle {
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn new(vertex1: Vector3<f32>, vertex2: Vector3<f32>, vertex3: Vector3<f32>,
        norm1: Vector3<f32>, norm2: Vector3<f32>, norm3: Vector3<f32>,
        uv1: Vector2<f32>, uv2: Vector2<f32>, uv3: Vector2<f32>,
        material: Arc<Material>, index: usize) -> Self
        {
        let mut tr = Triangle {
            vertex1, vertex2, vertex3,
            norm1, norm2, norm3,
            normal: Vector3::zeros(),
            uv1, uv2, uv3,
            material, index
        };
        tr.normal = tr.plane_normal();
        tr
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
    /// Barycentric coordinates
    pub fn bar_coords(&self, hit_point: &Vector3<f32>) -> Vector2<f32> {
        let v0v1: Vector3<f32> = self.vertex2 - self.vertex1;
        let v0v2: Vector3<f32> = self.vertex3 - self.vertex1;
        let n: Vector3<f32> = v0v1.cross(&v0v2);
        let denom = n.dot(&n);

        let mut c: Vector3<f32>;

        let edge1: Vector3<f32> = self.vertex3 - self.vertex2;
        let vp1: Vector3<f32> = hit_point - self.vertex2;
        c = edge1.cross(&vp1);
        let mut u = n.dot(&c);

        let edge2: Vector3<f32> = self.vertex1 - self.vertex3;
        let vp2: Vector3<f32> = hit_point - self.vertex3;
        c = edge2.cross(&vp2);
        let mut v = n.dot(&c);

        u /= denom;
        v /= denom;

        Vector2::new(u, v)
    }

    #[inline]
    pub fn vertex_color(&self, bar_coords: &Vector2<f32>) -> Vector3<f32> {
        let c1: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
        let c2: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
        let c3: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);

        bar_coords.x * c1 + bar_coords.y * c2 + (1.0 - bar_coords.x - bar_coords.y) * c3
    }

    #[inline]
    pub fn uv_coords(&self, bar_coords: &Vector2<f32>) -> Vector2<f32> {
        bar_coords.x * self.uv1 + bar_coords.y * self.uv2 + (1.0 - bar_coords.x - bar_coords.y) * self.uv3
    }
    
    #[inline]
    pub fn normal(&self, bar_coords: &Vector2<f32>, ray_direction: &Vector3<f32>) -> Vector3<f32> {
        let intrerp_normal: Vector3<f32> = bar_coords.x * self.norm1 + bar_coords.y * self.norm2 + (1.0 - bar_coords.x - bar_coords.y) * self.norm3;
        if intrerp_normal.dot(ray_direction) > 0.0 {
            -intrerp_normal
        } else {
            intrerp_normal
        }
    }

    #[inline]
    fn plane_normal(&self) -> Vector3<f32> {
        // Calculate the normal vector of the triangle (cross product of two edges)
        let v1: Vector3<f32> = self.vertex2 - self.vertex1;
        let v2: Vector3<f32> = self.vertex3 - self.vertex1;
        v1.cross(&v2).normalize()
    }

    #[inline]
    pub fn get_plane_normal(&self) -> Vector3<f32> {
        self.normal
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
}

impl Hittable<Triangle> for Triangle {
    // Möller–Trumbore intersection modified algorithm
    #[inline]
    #[allow(clippy::manual_range_contains)]
    fn intersect(&self, ray: &Ray) -> Option<Intersection<Self>> {
        const EPSILON: f32 = 0.0000001;
        let edge1 = self.vertex2 - self.vertex1;
        let edge2 = self.vertex3 - self.vertex1;
        let h = ray.get_direction().cross(&edge2);
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
        // This means that there is a line intersection but not a ray intersection.
        if t <= EPSILON || t.is_nan() {
            return None;
        }

        let v = f * ray.get_direction().dot(&q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        // If everything passed - we hit
        Some(Intersection::new(t, self))
    }
}

impl From<Triangle> for Bounds {
    #[inline]
    fn from(value: Triangle) -> Self {
        let points: [Vector3<f32>; 3] = [value.vertex1, value.vertex2, value.vertex3];

        let x_min = points.iter().map(|x: &Vector3<f32>| x.x).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let y_min = points.iter().map(|x: &Vector3<f32>| x.y).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let z_min = points.iter().map(|x: &Vector3<f32>| x.z).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();

        let x_max = points.iter().map(|x: &Vector3<f32>| x.x).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let y_max = points.iter().map(|x: &Vector3<f32>| x.y).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let z_max = points.iter().map(|x: &Vector3<f32>| x.z).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();

        let min_bounds = Vector3::new(x_min, y_min, z_min);
        let max_bounds = Vector3::new(x_max, y_max, z_max);
        Bounds {
            centroid: (value.vertex1 + value.vertex2 + value.vertex3) / 3.0,
            aabb_min: min_bounds,
            aabb_max: max_bounds
        }
    }
}

impl From<&Triangle> for Bounds {
    #[inline]
    fn from(value: &Triangle) -> Self {
        let points: [Vector3<f32>; 3] = [value.vertex1, value.vertex2, value.vertex3];

        let x_min = points.iter().map(|x: &Vector3<f32>| x.x).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let y_min = points.iter().map(|x: &Vector3<f32>| x.y).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let z_min = points.iter().map(|x: &Vector3<f32>| x.z).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();

        let x_max = points.iter().map(|x: &Vector3<f32>| x.x).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let y_max = points.iter().map(|x: &Vector3<f32>| x.y).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        let z_max = points.iter().map(|x: &Vector3<f32>| x.z).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();

        let min_bounds = Vector3::new(x_min, y_min, z_min);
        let max_bounds = Vector3::new(x_max, y_max, z_max);
        Bounds {
            centroid: (value.vertex1 + value.vertex2 + value.vertex3) / 3.0,
            aabb_min: min_bounds,
            aabb_max: max_bounds
        }
    }
}