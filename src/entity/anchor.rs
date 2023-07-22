use nalgebra::Vector3;

pub struct Anchor {
    pub position: Vector3<f32>
}

impl Anchor {
    #[inline]
    pub fn new(position: Vector3<f32>) -> Self {
        Anchor { position }
    }
}
