use std::ops::Index;

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix<const H: usize, const W: usize> {
    values: [[f32; W]; H],
}

#[macro_export]
macro_rules! m {
    ($($x:expr),+ $(,)?) => {{
        $crate::matrix::Matrix::new([$($x),*])
    }};
}

impl<const H: usize, const W: usize> Matrix<H, W> {
    pub const fn new(values: [[f32; W]; H]) -> Self {
        Self { values }
    }

    pub const fn zeros() -> Self {
        Self {
            values: [[0.; W]; H],
        }
    }

    pub const fn ones() -> Self {
        Self {
            values: [[1.; W]; H],
        }
    }

    pub const fn full(value: f32) -> Self {
        Self {
            values: [[value; W]; H],
        }
    }

    #[expect(clippy::unused_self)]
    pub const fn height(&self) -> usize {
        H
    }

    #[expect(clippy::unused_self)]
    pub const fn width(&self) -> usize {
        W
    }

    #[expect(clippy::unused_self)]
    pub const fn shape(&self) -> (usize, usize) {
        (H, W)
    }

    #[expect(clippy::unused_self)]
    pub const fn is_square(&self) -> bool {
        H == W
    }
}

impl<const H: usize, const W: usize> Index<(usize, usize)> for Matrix<H, W> {
    type Output = f32;

    fn index(&self, (y, x): (usize, usize)) -> &Self::Output {
        &self.values[y][x]
    }
}

macro_rules! impl_matrix_matrix {
    ($lhs:ty, $rhs:ty) => {
        impl<const H: usize, const W: usize> std::ops::Add<$rhs> for $lhs {
            type Output = Matrix<H, W>;

            fn add(self, rhs: $rhs) -> Matrix<H, W> {
                Matrix {
                    values: std::array::from_fn(|y| {
                        std::array::from_fn(|x| self.values[y][x] + rhs.values[y][x])
                    }),
                }
            }
        }

        impl<const H: usize, const W: usize> std::ops::Sub<$rhs> for $lhs {
            type Output = Matrix<H, W>;

            fn sub(self, rhs: $rhs) -> Matrix<H, W> {
                Matrix {
                    values: std::array::from_fn(|y| {
                        std::array::from_fn(|x| self.values[y][x] - rhs.values[y][x])
                    }),
                }
            }
        }
    };
}

impl_matrix_matrix!(Matrix<H, W>, Matrix<H, W>);
impl_matrix_matrix!(Matrix<H, W>, &Matrix<H, W>);
impl_matrix_matrix!(&Matrix<H, W>, Matrix<H, W>);
impl_matrix_matrix!(&Matrix<H, W>, &Matrix<H, W>);

macro_rules! impl_matrix_scalar {
    ($matrix:ty) => {
        impl<const H: usize, const W: usize> std::ops::Mul<f32> for $matrix {
            type Output = Matrix<H, W>;

            fn mul(self, scalar: f32) -> Self::Output {
                Matrix {
                    values: std::array::from_fn(|y| {
                        std::array::from_fn(|x| self.values[y][x] * scalar)
                    }),
                }
            }
        }

        impl<const H: usize, const W: usize> std::ops::Div<f32> for $matrix {
            type Output = Matrix<H, W>;

            fn div(self, scalar: f32) -> Self::Output {
                self * (1.0 / scalar)
            }
        }
    };
}

impl_matrix_scalar!(Matrix<H, W>);
impl_matrix_scalar!(&Matrix<H, W>);

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
            Matrix::<3, 2>::zeros().values,
            [[0., 0.], [0., 0.], [0., 0.]]
        );
        assert_eq!(Matrix::<2, 3>::ones(), Matrix::full(1.));
    }

    #[test]
    fn test_index() {
        let m = m![[0.25, 0.5, 1.], [2., 4., 8.]];
        assert_eq!(m[(0, 0)], 0.25);
        assert_eq!(m[(0, 1)], 0.5);
        assert_eq!(m[(0, 2)], 1.);
        assert_eq!(m[(1, 0)], 2.);
        assert_eq!(m[(1, 1)], 4.);
        assert_eq!(m[(1, 2)], 8.);
    }

    #[test]
    #[should_panic]
    fn test_index_invalid_row() {
        let m = m![[0.25, 0.5, 1.], [2., 4., 8.]];
        #[expect(clippy::no_effect)]
        m[(2, 0)];
    }

    #[test]
    #[should_panic]
    fn test_index_invalid_col() {
        let m = m![[0.25, 0.5, 1.], [2., 4., 8.]];
        #[expect(clippy::no_effect)]
        m[(0, 4)];
    }

    #[test]
    fn test_shape() {
        let m = Matrix::<2, 3>::zeros();
        assert_eq!(m.height(), 2);
        assert_eq!(m.width(), 3);
        assert_eq!(m.shape(), (2, 3));
    }

    #[test]
    fn test_is_square() {
        assert!(!Matrix::<2, 3>::zeros().is_square());
        assert!(Matrix::<6, 6>::zeros().is_square());
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Matrix::<4, 2>::zeros() + Matrix::<4, 2>::zeros(),
            Matrix::<4, 2>::zeros()
        );
        assert_eq!(
            m![[1., 2.], [3., 4.]] + &m![[-1., -2.], [-3., -4.]],
            Matrix::zeros()
        );
        assert_eq!(
            &m![[0.5, 1., 1.5], [2., 2.5, 3.]] + m![[2.5, 2., 1.5], [1., 0.5, 0.]],
            Matrix::full(3.)
        );
        assert_eq!(&m![[1., 2.5, 0.]] + &m![[3., -4., 0.]], m![[4., -1.5, 0.]]);
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Matrix::<4, 2>::zeros() - Matrix::<4, 2>::zeros(),
            Matrix::<4, 2>::zeros()
        );
        assert_eq!(
            m![[1., 2.], [3., 4.]] - &m![[1., 2.], [3., 4.]],
            Matrix::zeros()
        );
        assert_eq!(
            &m![[0.5, 1., 1.5], [2., 2.5, 3.]] - m![[1.5, 2., 2.5], [3., 3.5, 4.]],
            Matrix::full(-1.)
        );
        assert_eq!(&m![[1., 2.5, 0.]] - &m![[3., -4., 0.]], m![[-2., 6.5, 0.]]);
    }

    #[test]
    fn test_scalar_mul() {
        assert_eq!(Matrix::<4, 2>::zeros() * 7., Matrix::zeros());
        assert_eq!(m![[1., 2.]] * 3., m![[3., 6.]]);
        assert_eq!(&m![[1.], [2.]] * -2.5, m![[-2.5], [-5.]]);
        assert_eq!(&Matrix::<4, 2>::full(2.5) * 2.5, Matrix::full(6.25));
    }
}
