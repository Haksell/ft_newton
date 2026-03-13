use crate::{quaternion::Quaternion, sphere::Sphere, vector::Vector};

#[derive(Default, Debug)]
pub struct Body {
    pub position: Vector<3>,
    pub orientation: Quaternion,
    pub sphere: Sphere, // TODO: sphere -> trait or enum shape
}
