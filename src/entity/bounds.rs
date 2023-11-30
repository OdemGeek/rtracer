use nalgebra::Vector3;

#[derive(Debug, Default)]
pub struct Bounds {
    pub centroid: Vector3<f32>,
    pub aabb_min: Vector3<f32>,
    pub aabb_max: Vector3<f32>,
}

impl Bounds {
    #[inline(always)]
    pub fn new(centroid: Vector3<f32>, aabb_min: Vector3<f32>, aabb_max: Vector3<f32>) -> Self {
        Bounds { centroid, aabb_min, aabb_max }
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::Vector3;
    use crate::{entity::triangle::Triangle, material::Material};

    use super::Bounds;

    #[test]
    fn triangle_bounds_from_points() {
        let triangle = Bounds::from(
            Triangle::new(
            Vector3::new(-0.5, 0.0, 0.0),
            Vector3::new(1.2, 0.0, -0.25),
            Vector3::new(0.0, 1.0, 0.0),
            Material::default().into()
            )
        );
        assert_eq!(triangle.aabb_min, Vector3::new(-0.5, 0.0, -0.25));
        assert_eq!(triangle.aabb_max, Vector3::new(1.2, 1.0, 0.0));
    }
}