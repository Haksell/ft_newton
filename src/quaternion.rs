use crate::vector::Vector;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Quaternion {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn axis_angle(axis: &Vector<3>, angle: f32) -> Self {
        let half_angle_radians = angle * 0.5;
        let w = half_angle_radians.cos();
        let half_sine = half_angle_radians.sin();
        let axis = axis.normalize();
        let x = axis.x() * half_sine;
        let y = axis.y() * half_sine;
        let z = axis.z() * half_sine;
        Self { x, y, z, w }
    }

    pub const fn zero() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
            w: 1.,
        }
    }

    pub const fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    // normalize
    // invert
    // inverse
    // rotate_point
    // is_valid
    // rotate_matrix
    // to_mat3
}

macro_rules! impl_quaternion_quaternion {
    ($lhs:ty, $rhs:ty) => {
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = Quaternion;

            fn mul(self, rhs: $rhs) -> Self::Output {
                Quaternion {
                    x: self.x * rhs.w + self.w * rhs.x + self.y * rhs.z - self.z * rhs.y,
                    y: self.y * rhs.w + self.w * rhs.y + self.z * rhs.x - self.x * rhs.z,
                    z: self.z * rhs.w + self.w * rhs.z + self.x * rhs.y - self.y * rhs.x,
                    w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
                }
            }
        }
    };
}

impl_quaternion_quaternion!(Quaternion, Quaternion);
impl_quaternion_quaternion!(Quaternion, &Quaternion);
impl_quaternion_quaternion!(&Quaternion, Quaternion);
impl_quaternion_quaternion!(&Quaternion, &Quaternion);

macro_rules! impl_quaternion_scalar {
    ($quaternion:ty) => {
        impl std::ops::Mul<f32> for $quaternion {
            type Output = Quaternion;

            fn mul(self, scalar: f32) -> Self::Output {
                Quaternion {
                    x: self.x * scalar,
                    y: self.y * scalar,
                    z: self.z * scalar,
                    w: self.w * scalar,
                }
            }
        }
    };
}

impl_quaternion_scalar!(Quaternion);
impl_quaternion_scalar!(&Quaternion);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructors() {
        assert_eq!(Quaternion::zero(), Quaternion::new(0., 0., 0., 1.));
    }
}
