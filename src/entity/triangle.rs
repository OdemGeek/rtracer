use std::sync::Arc;
use nalgebra::Vector3;
use crate::{math::ray::Ray, material::Material};
use super::{hittable::Hittable, hit::Hit};

// Maybe change it to pointer to vertex slice of vertexes
pub struct Triangle {
    vertex1: Vector3<f32>,
    vertex2: Vector3<f32>,
    vertex3: Vector3<f32>,
    normal: Vector3<f32>,
    pub material: Arc<Material>,
}

#[allow(dead_code)]
impl Triangle {
    pub fn new(vertex1: Vector3<f32>, vertex2: Vector3<f32>, vertex3: Vector3<f32>, material: Arc<Material>) -> Self {
        let mut x = Triangle {
            vertex1, vertex2, vertex3, material,
            normal: Vector3::zeros(),
        };
        x.normal = x.plane_normal();
        x
    }

    // Code provided by ChatGPT
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

    pub fn normal_flipped(&self, ray_direction: &Vector3<f32>) -> Vector3<f32> {
        let direction = self.normal.dot(ray_direction);
        let is_flipped = if direction > 0.0 {-1.0} else {1.0};
        self.normal * is_flipped
    }
}

#[allow(dead_code)]
impl Hittable for Triangle {
    // Code provided by ChatGPT
    fn intersect(&self, ray: &Ray) -> Option<super::hit::Hit> {
        // Calculate the normal vector of the triangle
        // Change to `normal()` after smooth normals implementation
        let triangle_normal = self.normal_flipped(&ray.direction);
    
        // Check if the ray is parallel to the triangle (dot product of ray direction and normal)
        let ray_dir_dot_normal = ray.direction.dot(&triangle_normal);
        if ray_dir_dot_normal.abs() < 1e-6 {
            return None; // Ray is parallel to the triangle, no intersection
        }
    
        // Calculate the distance from the ray's origin to the triangle plane
        let t = triangle_normal.dot(&(self.vertex1 - ray.origin)) / ray_dir_dot_normal;
    
        if t < 0.0 {
            return None; // Triangle is behind the ray's origin, no intersection
        }
    
        // Calculate the intersection point
        let intersection_point = ray.origin + t * ray.direction;
    
        // Check if the intersection point is inside the triangle using barycentric coordinates
        let edge1 = self.vertex2 - self.vertex1;
        let edge2 = self.vertex3 - self.vertex1;
        let edge3 = intersection_point - self.vertex1;
    
        let dot11 = edge1.dot(&edge1);
        let dot12 = edge1.dot(&edge2);
        let dot22 = edge2.dot(&edge2);
        let dot13 = edge1.dot(&edge3);
        let dot23 = edge2.dot(&edge3);
    
        let denom = dot11 * dot22 - dot12 * dot12;
    
        let u = (dot22 * dot13 - dot12 * dot23) / denom;
        let v = (dot11 * dot23 - dot12 * dot13) / denom;
    
        // Check if the intersection point is inside the triangle
        if u >= 0.0 && v >= 0.0 && u + v <= 1.0 {
            Some(Hit::new(t, intersection_point, triangle_normal, self)) // Intersection point is inside the triangle
        } else {
            None // Intersection point is outside the triangle
        }
    }

    fn normal(&self, ray_direction: &Vector3<f32>) -> Vector3<f32> {
        self.normal_flipped(ray_direction)
    }
}
