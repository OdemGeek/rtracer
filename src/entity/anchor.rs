use nalgebra::{Vector3, Rotation3, UnitVector3};

pub struct Anchor {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    rotation_matrix: Rotation3<f32>,
    forward: UnitVector3<f32>,
    right: UnitVector3<f32>,
    up: UnitVector3<f32>,
}

#[allow(dead_code)]
impl Anchor {
    #[inline]
    /// Rotation is in euler angles (radians).
    pub fn new(position: Vector3<f32>, rotation: Vector3<f32>) -> Self {
        let rotation_matrix = Rotation3::from_euler_angles(rotation.x, rotation.y, rotation.z);
        Anchor { position, rotation,
            rotation_matrix,
            forward: rotation_matrix * Vector3::z_axis(),
            right: rotation_matrix * Vector3::x_axis(),
            up: rotation_matrix * Vector3::y_axis(),
        }
    }

    #[inline]
    fn init(&mut self) {
        self.forward = self.rotation_matrix * Vector3::z_axis();
        self.right = self.rotation_matrix * Vector3::x_axis();
        self.up = self.rotation_matrix * Vector3::y_axis();
    }

    #[inline]
    pub fn forward(&self) -> UnitVector3<f32> {
        self.forward
    }

    #[inline]
    pub fn right(&self) -> UnitVector3<f32> {
        self.right
    }

    #[inline]
    pub fn up(&self) -> UnitVector3<f32> {
        self.up
    }

    #[inline]
    pub fn set_rotation(&mut self, rotation: Vector3<f32>) {
        self.rotation = rotation;
        self.rotation_matrix = Rotation3::from_euler_angles(rotation.x, rotation.y, rotation.z);
        self.init()
    }

    #[inline]
    pub fn translate(&mut self, translation: Vector3<f32>) {
        self.position += translation;
    }
    
    #[inline]
    pub fn translate_relative(&mut self, translation: Vector3<f32>) {
        self.position = self.position + self.forward.into_inner() * translation.z + self.right.into_inner() * translation.x + self.up.into_inner() * translation.y;
    }
}
