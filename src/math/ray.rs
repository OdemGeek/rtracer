use nalgebra::Vector3;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub struct Ray {
    pub origin: Vector3<f32>,
    direction: Vector3<f32>,
    frac_direction: Vector3<f32>,
}

#[allow(dead_code)]
impl Ray {
    #[inline]
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Self {
        Ray {
            origin,
            direction,
            frac_direction: Vector3::new(1.0 / direction.x, 1.0 / direction.y, 1.0 / direction.z)
        }
    }
    
    #[inline]
    pub fn set_direction(&mut self, direction: &Vector3<f32>) {
        self.direction = *direction;
        self.frac_direction = Vector3::new(1.0 / direction.x, 1.0 / direction.y, 1.0 / direction.z);
    }

    #[inline(always)]
    pub fn get_direction(&self) -> &Vector3<f32> {
        &self.direction
    }

    #[inline(always)]
    pub fn get_frac_direction(&self) -> &Vector3<f32> {
        &self.frac_direction
    }
}