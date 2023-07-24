use super::triangle::Triangle;

pub struct Hit<'a> {
    pub t: f32,
    pub object: &'a Triangle,
}

#[allow(dead_code)]
impl<'a> Hit<'a> {
    pub fn new(t: f32, object: &'a Triangle) -> Self {
        Hit { t, object }
    }
}
