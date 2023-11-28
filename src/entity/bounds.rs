use nalgebra::Vector3;

#[derive(Debug)]
pub struct Bounds {
    pub centroid: Vector3<f32>,
    pub aabb_min: Vector3<f32>,
    pub aabb_max: Vector3<f32>,
}

impl Bounds {
    #[inline]
    pub fn new(point1: Vector3<f32>, point2: Vector3<f32>, point3: Vector3<f32>) -> Self {
        let (min_bounds, max_bounds) = Self::bounds_from_points(point1, point2, point3);
        Bounds {
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
    use super::Bounds;

    #[test]
    fn triangle_bounds_from_points() {
        let triangle = Bounds::new(
            Vector3::new(-0.5, 0.0, 0.0),
            Vector3::new(1.2, 0.0, -0.25),
            Vector3::new(0.0, 1.0, 0.0)
        );
        assert_eq!(triangle.aabb_min, Vector3::new(-0.5, 0.0, -0.25));
        assert_eq!(triangle.aabb_max, Vector3::new(1.2, 1.0, 0.0));
    }
}