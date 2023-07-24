use std::sync::Arc;
use nalgebra::Vector3;
use crate::{math::ray::Ray, material::Material};
use super::{hittable::Hittable, hit::Hit};

// Maybe change it to pointer to vertex slice of vertexes
pub struct Triangle {
    pub vertex1: Vector3<f32>,
    pub vertex2: Vector3<f32>,
    pub vertex3: Vector3<f32>,
    normal: Vector3<f32>,
    pub material: Arc<Material>,
}

#[allow(dead_code)]
impl Triangle {
    pub fn new(vertex1: Vector3<f32>, vertex2: Vector3<f32>, vertex3: Vector3<f32>, material: Arc<Material>) -> Self {
        let mut x = Triangle { vertex1, vertex2, vertex3, material, normal: Vector3::zeros() };
        x.normal = x.plane_normal();
        x
    }

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
}

#[allow(dead_code)]
impl Hittable for Triangle {
    // Code provided by ChatGPT
    fn intersect(&self, ray: &Ray) -> Option<super::hit::Hit> {
        // Calculate the normal vector of the triangle
        // Change to `normal()` after smooth normals implementation
        let triangle_normal = self.normal;
    
        // Check if the ray is parallel to the triangle (dot product of ray direction and normal)
        let ray_dir_dot_normal = ray.direction.dot(&triangle_normal);
        if ray_dir_dot_normal.abs() < 1e-6 {
            return None; // Ray is parallel to the triangle, no intersection
        }
    
        // Calculate the distance from the ray's origin to the triangle plane
        let t = Vector3::dot(
            &Vector3::new(
                self.vertex1.x - ray.origin.x,
                self.vertex1.y - ray.origin.y,
                self.vertex1.z - ray.origin.z,
            ),
            &triangle_normal,
        ) / ray_dir_dot_normal;
    
        if t < 0.0 {
            return None; // Triangle is behind the ray's origin, no intersection
        }
    
        // Calculate the intersection point
        let intersection_point = Vector3::new(
            ray.origin.x + t * ray.direction.x,
            ray.origin.y + t * ray.direction.y,
            ray.origin.z + t * ray.direction.z,
        );
    
        // Check if the intersection point is inside the triangle using barycentric coordinates
        let edge1 = Vector3::new(
            self.vertex2.x - self.vertex1.x,
            self.vertex2.y - self.vertex1.y,
            self.vertex2.z - self.vertex1.z,
        );
        let edge2 = Vector3::new(
            self.vertex3.x - self.vertex1.x,
            self.vertex3.y - self.vertex1.y,
            self.vertex3.z - self.vertex1.z,
        );
        let edge3 = Vector3::new(
            intersection_point.x - self.vertex1.x,
            intersection_point.y - self.vertex1.y,
            intersection_point.z - self.vertex1.z,
        );
    
        let dot11 = Vector3::dot(&edge1, &edge1);
        let dot12 = Vector3::dot(&edge1, &edge2);
        let dot22 = Vector3::dot(&edge2, &edge2);
        let dot13 = Vector3::dot(&edge1, &edge3);
        let dot23 = Vector3::dot(&edge2, &edge3);
    
        let denom = dot11 * dot22 - dot12 * dot12;
    
        let u = (dot22 * dot13 - dot12 * dot23) / denom;
        let v = (dot11 * dot23 - dot12 * dot13) / denom;
    
        // Check if the intersection point is inside the triangle
        if u >= 0.0 && v >= 0.0 && u + v <= 1.0 {
            Some(Hit::new(t, self)) // Intersection point is inside the triangle
        } else {
            None // Intersection point is outside the triangle
        }
    }

    // Code provided by ChatGPT
    fn normal(&self, _point: &Vector3<f32>) -> Vector3<f32> {
        self.plane_normal()
    }
}
