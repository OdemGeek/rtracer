use super::vectors::Float3;

#[allow(dead_code)]
pub struct Ray {
    pub origin: Float3,
    pub direction: Float3,
}

#[allow(dead_code)]
impl Ray {
    pub fn new(origin: Float3, direction: Float3) -> Self {
        Ray { origin, direction }
    }
}