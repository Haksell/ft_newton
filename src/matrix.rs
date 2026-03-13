use {crate::vector::Vector, std::ops::Index};

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix<const N: usize> {
    values: [[f32; N]; N],
}

#[macro_export]
macro_rules! m {
    ($($x:expr),+ $(,)?) => {{
        $crate::matrix::Matrix::new([$($x),*])
    }};
}

impl<const N: usize> Matrix<N> {
    pub const fn new(values: [[f32; N]; N]) -> Self {
        Self { values }
    }

    pub const fn zeros() -> Self {
        Self {
            values: [[0.; N]; N],
        }
    }

    pub const fn ones() -> Self {
        Self {
            values: [[1.; N]; N],
        }
    }

    pub const fn full(value: f32) -> Self {
        Self {
            values: [[value; N]; N],
        }
    }

    pub const fn identity() -> Self {
        let mut mat = Self::zeros();
        // for loops are not const in stable
        let mut i = 0;
        while i < N {
            mat.values[i][i] = 1.;
            i += 1;
        }
        mat
    }

    #[expect(clippy::unused_self)]
    pub const fn size(&self) -> usize {
        N
    }

    pub const fn row(&self, y: usize) -> Vector<N> {
        Vector::new(self.values[y])
    }

    pub fn col(&self, x: usize) -> Vector<N> {
        Vector::new(std::array::from_fn(|y| self.values[y][x]))
    }
}

impl<const N: usize> Index<(usize, usize)> for Matrix<N> {
    type Output = f32;

    fn index(&self, (y, x): (usize, usize)) -> &Self::Output {
        &self.values[y][x]
    }
}

macro_rules! impl_matrix_matrix {
    ($lhs:ty, $rhs:ty) => {
        impl<const N: usize> std::ops::Add<$rhs> for $lhs {
            type Output = Matrix<N>;

            fn add(self, rhs: $rhs) -> Matrix<N> {
                Matrix {
                    values: std::array::from_fn(|y| {
                        std::array::from_fn(|x| self.values[y][x] + rhs.values[y][x])
                    }),
                }
            }
        }

        impl<const N: usize> std::ops::Sub<$rhs> for $lhs {
            type Output = Matrix<N>;

            fn sub(self, rhs: $rhs) -> Matrix<N> {
                Matrix {
                    values: std::array::from_fn(|y| {
                        std::array::from_fn(|x| self.values[y][x] - rhs.values[y][x])
                    }),
                }
            }
        }
    };
}

impl_matrix_matrix!(Matrix<N>, Matrix<N>);
impl_matrix_matrix!(Matrix<N>, &Matrix<N>);
impl_matrix_matrix!(&Matrix<N>, Matrix<N>);
impl_matrix_matrix!(&Matrix<N>, &Matrix<N>);

macro_rules! impl_matrix_scalar {
    ($matrix:ty) => {
        impl<const N: usize> std::ops::Mul<f32> for $matrix {
            type Output = Matrix<N>;

            fn mul(self, scalar: f32) -> Self::Output {
                Matrix {
                    values: std::array::from_fn(|y| {
                        std::array::from_fn(|x| self.values[y][x] * scalar)
                    }),
                }
            }
        }

        impl<const N: usize> std::ops::Div<f32> for $matrix {
            type Output = Matrix<N>;

            fn div(self, scalar: f32) -> Self::Output {
                self * (1.0 / scalar)
            }
        }
    };
}

impl_matrix_scalar!(Matrix<N>);
impl_matrix_scalar!(&Matrix<N>);

#[expect(clippy::float_cmp)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructors() {
        assert_eq!(
            Matrix::new([[1., 2.], [3.14, 4.2]]),
            m![[1., 2.], [3.14, 4.2]]
        );
        assert_eq!(
            Matrix::<3>::zeros().values,
            [[0., 0., 0.], [0., 0., 0.], [0., 0., 0.]]
        );
        assert_eq!(Matrix::<2>::ones(), Matrix::full(1.));
    }

    #[test]
    fn test_index() {
        let m = m![[0.25, 0.5], [2., 4.]];
        assert_eq!(m[(0, 0)], 0.25);
        assert_eq!(m[(0, 1)], 0.5);
        assert_eq!(m[(1, 0)], 2.);
        assert_eq!(m[(1, 1)], 4.);
    }

    #[test]
    #[should_panic]
    fn test_index_invalid_row() {
        let m = m![[0.25, 0.5], [2., 4.]];
        #[expect(clippy::no_effect)]
        m[(2, 0)];
    }

    #[test]
    #[should_panic]
    fn test_index_invalid_col() {
        let m = m![[0.25, 0.5], [2., 4.]];
        #[expect(clippy::no_effect)]
        m[(0, 4)];
    }

    #[test]
    fn test_size() {
        assert_eq!(Matrix::<3>::zeros().size(), 3);
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Matrix::<2>::zeros() + Matrix::<2>::zeros(),
            Matrix::<2>::zeros()
        );
        assert_eq!(
            m![[1., 2.], [3., 4.]] + &m![[-1., -2.], [-3., -4.]],
            Matrix::zeros()
        );
        assert_eq!(
            &m![[0.5, 1.], [2., 2.5]] + m![[2.5, 2.], [1., 0.5]],
            Matrix::full(3.)
        );
        assert_eq!(
            &m![[1., 2.5, 0.], [1., 2.5, 0.], [1., 2.5, 0.],]
                + &m![[3., -4., 0.], [3., -4., 0.], [3., -4., 42.]],
            m![[4., -1.5, 0.], [4., -1.5, 0.], [4., -1.5, 42.],]
        );
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Matrix::<2>::zeros() - Matrix::<2>::zeros(),
            Matrix::<2>::zeros()
        );
        assert_eq!(
            m![[1., 2.], [3., 4.]] - &m![[1., 2.], [3., 4.]],
            Matrix::zeros()
        );
        assert_eq!(
            &m![[0.5, 1.], [2., 2.5]] - m![[1.5, 2.], [3., 3.5]],
            Matrix::full(-1.)
        );
        assert_eq!(
            &m![[1., 2.5, 0.], [1., 2.5, 0.], [1., 2.5, 0.]]
                - &m![[3., -4., 0.], [3., -4., 0.], [3., -4., 42.]],
            m![[-2., 6.5, 0.], [-2., 6.5, 0.], [-2., 6.5, -42.],]
        );
    }

    #[test]
    fn test_scalar_mul() {
        assert_eq!(Matrix::<2>::zeros() * 7., Matrix::zeros());
        assert_eq!(m![[1., 2.], [3., 4.]] * 3., m![[3., 6.], [9., 12.]]);
        assert_eq!(&Matrix::<4>::full(2.5) * 2.5, Matrix::full(6.25));
    }
}
