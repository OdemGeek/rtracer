use super::sphere::Sphere;

pub struct Hit<'a> {
    pub t: f32,
    pub object: &'a Sphere,
}

#[allow(dead_code)]
impl<'a> Hit<'a> {
    pub fn new(t: f32, object: &'a Sphere) -> Self {
        Hit { t, object }
    }
}
