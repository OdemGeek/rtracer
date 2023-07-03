use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy)]
pub struct Float2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub trait Vector {
    fn magnitude(&self) -> f32;
    fn normalized(&self) -> Self;
    fn dot(first: &Self, second: &Self) -> f32;
}

#[allow(dead_code)]
impl Float2 {
    pub fn new(x: f32, y: f32) -> Self {
        Float2 { x, y }
    }

    pub fn zero() -> Self {
        Float2 { x: 0.0, y: 0.0 }
    }
    
    pub fn one() -> Self {
        Float2 { x: 1.0, y: 1.0 }
    }
}

#[allow(dead_code)]
impl Float3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Float3 { x, y, z }
    }

    pub fn zero() -> Self {
        Float3 { x: 0.0, y: 0.0, z: 0.0 }
    }
    
    pub fn one() -> Self {
        Float3 { x: 1.0, y: 1.0, z: 1.0 }
    }

    pub fn cross(first: &Float3, second: &Float3) -> Float3 {
        Float3 {
            x: first.y * second.z - first.z * second.y,
            y: first.z * second.x - first.x * second.z,
            z: first.x * second.y - first.y * second.x,
        }
    }
}

impl Vector for Float2 {
    fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    fn normalized(&self) -> Self {
        let magnitude = self.magnitude();
        Float2 {
            x: self.x / magnitude,
            y: self.y / magnitude,
        }
    }

    fn dot(first: &Float2, second: &Float2) -> f32 {
        first.x * second.x + first.y * second.y
    }
}

impl Vector for Float3 {
    fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    fn normalized(&self) -> Self {
        let magnitude = self.magnitude();
        Float3 {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }

    fn dot(first: &Float3, second: &Float3) -> f32 {
        first.x * second.x + first.y * second.y + first.z * second.z
    }
}

impl Add for Float2 {
    type Output = Float2;

    fn add(self, other: Float2) -> Float2 {
        Float2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add for Float3 {
    type Output = Float3;

    fn add(self, other: Float3) -> Float3 {
        Float3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Float2 {
    type Output = Float2;

    fn sub(self, other: Float2) -> Float2 {
        Float2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub for Float3 {
    type Output = Float3;

    fn sub(self, other: Float3) -> Float3 {
        Float3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Float2 {
    type Output = Float2;

    fn mul(self, scalar: f32) -> Float2 {
        Float2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Mul<Float2> for f32 {
    type Output = Float2;

    fn mul(self, vector: Float2) -> Float2 {
        Float2 {
            x: vector.x * self,
            y: vector.y * self,
        }
    }
}

impl Mul<f32> for Float3 {
    type Output = Float3;

    fn mul(self, scalar: f32) -> Float3 {
        Float3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Float3> for f32 {
    type Output = Float3;

    fn mul(self, vector: Float3) -> Float3 {
        Float3 {
            x: vector.x * self,
            y: vector.y * self,
            z: vector.z * self,
        }
    }
}

impl Div<f32> for Float2 {
    type Output = Float2;

    fn div(self, scalar: f32) -> Float2 {
        Float2 {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl Div<f32> for Float3 {
    type Output = Float3;

    fn div(self, scalar: f32) -> Float3 {
        Float3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}
