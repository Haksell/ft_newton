use crate::body::Body;

#[derive(Debug)]
pub struct Scene {
    bodies: Vec<Body>,
}

impl Scene {
    // TODO: remove
    pub fn manual() -> Self {
        Self {
            bodies: vec![Body::default()],
        }
    }
}
