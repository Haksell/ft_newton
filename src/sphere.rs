#[derive(Debug)]
pub struct Sphere {
    radius: f32,
}

impl Default for Sphere {
    fn default() -> Self {
        Self { radius: 1. }
    }
}
