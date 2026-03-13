use std::ops::Index;

#[derive(Clone, Debug, PartialEq)]
pub struct Vector<const N: usize> {
    values: [f32; N],
}

#[macro_export]
macro_rules! v {
    ($($x:expr),+ $(,)?) => {{
        $crate::vector::Vector::new([$($x),*])
    }};
}

impl<const N: usize> Vector<N> {
    pub const fn new(values: [f32; N]) -> Self {
        Self { values }
    }

    pub const fn zeros() -> Self {
        Self { values: [0.0; N] }
    }

    pub const fn ones() -> Self {
        Self { values: [1.0; N] }
    }

    pub const fn full(value: f32) -> Self {
        Self { values: [value; N] }
    }

    #[expect(clippy::unused_self)]
    pub const fn len(&self) -> usize {
        N
    }

    pub fn normalize(&mut self) {
        *self /= self.magnitude();
    }

    pub fn normalized(&self) -> Self {
        self / self.magnitude()
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        (0..N).map(|i| self[i] * rhs[i]).sum()
    }

    pub fn magnitude_squared(&self) -> f32 {
        self.dot(self)
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }
}

impl<const N: usize> Index<usize> for Vector<N> {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

macro_rules! impl_vector_vector {
    ($lhs:ty, $rhs:ty) => {
        impl<const N: usize> std::ops::Add<$rhs> for $lhs {
            type Output = Vector<N>;

            fn add(self, rhs: $rhs) -> Vector<N> {
                Vector {
                    values: std::array::from_fn(|i| self.values[i] + rhs.values[i]),
                }
            }
        }

        impl<const N: usize> std::ops::Sub<$rhs> for $lhs {
            type Output = Vector<N>;

            fn sub(self, rhs: $rhs) -> Vector<N> {
                Vector {
                    values: std::array::from_fn(|i| self.values[i] - rhs.values[i]),
                }
            }
        }
    };
}

impl_vector_vector!(Vector<N>, Vector<N>);
impl_vector_vector!(Vector<N>, &Vector<N>);
impl_vector_vector!(&Vector<N>, Vector<N>);
impl_vector_vector!(&Vector<N>, &Vector<N>);

macro_rules! impl_vector_scalar {
    ($vector:ty) => {
        impl<const N: usize> std::ops::Mul<f32> for $vector {
            type Output = Vector<N>;

            fn mul(self, scalar: f32) -> Self::Output {
                Vector {
                    values: std::array::from_fn(|i| self.values[i] * scalar),
                }
            }
        }

        impl<const N: usize> std::ops::Div<f32> for $vector {
            type Output = Vector<N>;

            fn div(self, scalar: f32) -> Self::Output {
                self * (1.0 / scalar)
            }
        }
    };
}

impl_vector_scalar!(Vector<N>);
impl_vector_scalar!(&Vector<N>);

impl<const N: usize> std::ops::MulAssign<f32> for Vector<N> {
    fn mul_assign(&mut self, rhs: f32) {
        for x in &mut self.values {
            *x *= rhs;
        }
    }
}

impl<const N: usize> std::ops::DivAssign<f32> for Vector<N> {
    fn div_assign(&mut self, rhs: f32) {
        *self *= 1.0 / rhs;
    }
}

impl Vector<3> {
    pub fn cross(&self, rhs: &Self) -> Self {
        v![
            self[1] * rhs[2] - self[2] * rhs[1],
            self[2] * rhs[0] - self[0] * rhs[2],
            self[0] * rhs[1] - self[1] * rhs[0],
        ]
    }

    pub const fn x(&self) -> f32 {
        self.values[0]
    }

    pub const fn y(&self) -> f32 {
        self.values[1]
    }

    pub const fn z(&self) -> f32 {
        self.values[2]
    }
}

#[expect(clippy::float_cmp)]
#[cfg(test)]
mod tests {
    use std::f32::consts::FRAC_1_SQRT_2;

    use super::*;

    #[test]
    fn test_constructors() {
        assert_eq!(Vector::new([1., 2., 3.14, 4.2]), v![1., 2., 3.14, 4.2]);
        assert_eq!(Vector::<3>::zeros().values, [0., 0., 0.]);
        assert_eq!(Vector::<3>::ones(), Vector::full(1.));
    }

    #[test]
    fn test_len() {
        assert_eq!(v![1., 2., 3., 4.].len(), 4);
    }

    #[test]
    fn test_index() {
        let v = v![1., 2., 3.];
        assert_eq!(v[0], 1.);
        assert_eq!(v[1], 2.);
        assert_eq!(v[2], 3.);
    }

    #[test]
    #[should_panic]
    fn test_index_invalid() {
        let v = v![1., 2., 3.];
        #[expect(clippy::no_effect)]
        v[3];
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Vector::<42>::zeros() + Vector::<42>::zeros(),
            Vector::<42>::zeros()
        );
        assert_eq!(v![1., 2.] + &v![3., 4.], v![4., 6.]);
        assert_eq!(&v![1., 2.] + v![3., 4.], v![4., 6.]);
        assert_eq!(&v![1., 2.5, 0.] + &v![3., -4., 0.], v![4., -1.5, 0.]);
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Vector::<42>::zeros() - Vector::<42>::zeros(),
            Vector::<42>::zeros()
        );
        assert_eq!(v![1., 2.] - &v![3., 4.], v![-2., -2.]);
        assert_eq!(&v![1., 2.] - v![3., 4.], v![-2., -2.]);
        assert_eq!(&v![1., 2.5, 0.] - &v![3., -4., 0.], v![-2., 6.5, 0.]);
    }

    #[test]
    fn test_scalar_mul() {
        assert_eq!(Vector::<42>::zeros() * 7., Vector::zeros());
        assert_eq!(v![1., 2.] * 3., v![3., 6.]);
        assert_eq!(&v![1., 2.] * -2.5, v![-2.5, -5.]);
        assert_eq!(&v![1., 2.5, 0.] * 0., v![0., 0., 0.]);
    }

    #[test]
    fn test_dot_product() {
        assert_eq!(Vector::<2>::zeros().dot(&Vector::<2>::ones()), 0.);
        assert_eq!(Vector::<2>::ones().dot(&Vector::<2>::ones()), 2.);
        assert_eq!(v![-1., 6.].dot(&v![3., 2.]), 9.);
    }

    #[test]
    fn test_cross() {
        assert_eq!(v![2., 0., 0.].cross(&v![0., 3., 0.]), v![0., 0., 6.]);
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(Vector::<42>::zeros().magnitude(), 0.);
        assert_eq!(v![3., -4., 0.].magnitude(), 5.);
    }

    #[test]
    fn test_normalize() {
        let mut v = v![1., 1.];
        v.normalize();
        assert_eq!(v, v![FRAC_1_SQRT_2, FRAC_1_SQRT_2]);
        assert_eq!(v![3., -4., 0.].normalized(), v![0.6, -0.8, 0.0]);
    }
}
