use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use crate::number::Number;

impl<T> Matrix<T> {
    pub fn new(v: Vec<Vec<T>>) -> Self {
        Self {
            h: v.len(),
            w: v[0].len(),
            v,
        }
    }

    pub fn identity(n: usize) -> Self
    where
        T: Number + Clone,
    {
        let mut v = vec![vec![T::zero(); n]; n];
        for i in 0..n {
            v[i][i] = T::one();
        }

        Matrix::<T>::new(v)
    }

    pub fn mul_vec(&mut self, rhs: Vec<T>) -> Vec<T>
    where
        T: Mul<Output = T> + AddAssign + Number + Clone + Copy,
    {
        let mut v = vec![T::zero(); rhs.len()];
        for i in 0..self.h {
            for j in 0..self.w {
                v[i] += self.v[i][j] * rhs[j];
            }
        }
        v
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T> {
    h: usize,
    w: usize,
    v: Vec<Vec<T>>,
}

impl<T> AddAssign for Matrix<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..self.h {
            for j in 0..self.w {
                self.v[i][j] += rhs.v[i][j];
            }
        }
    }
}

impl<T> SubAssign for Matrix<T>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..self.h {
            for j in 0..self.w {
                self.v[i][j] -= rhs.v[i][j];
            }
        }
    }
}

impl<T> MulAssign for Matrix<T>
where
    T: AddAssign + Mul<Output = T> + Number + Copy,
{
    fn mul_assign(&mut self, rhs: Self) {
        let mut v = vec![vec![T::zero(); rhs.w]; self.h];
        for i in 0..self.h {
            for j in 0..rhs.w {
                for k in 0..rhs.h {
                    v[i][j] += self.v[i][k] * rhs.v[k][j];
                }
            }
        }
        *self = Matrix::new(v);
    }
}

impl<T> Add for Matrix<T>
where
    T: AddAssign + Copy,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res += rhs;
        res
    }
}

impl<T> Mul for Matrix<T>
where
    T: AddAssign + Mul<Output = T> + Number + Copy,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res *= rhs;
        res
    }
}

impl<T> Sub for Matrix<T>
where
    T: SubAssign + Copy,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res -= rhs;
        res
    }
}

#[cfg(test)]
mod test {
    use crate::matrix::Matrix;

    #[test]
    fn test_matrix_add() {}

    #[test]
    fn test_matrix_mul() {
        let a = vec![vec![1, -1], vec![-2, 3]];
        let b = vec![vec![1, 2], vec![3, 4]];
        let c = vec![vec![-2, -2], vec![7, 8]];
        assert_eq!(Matrix::new(a) * Matrix::new(b), Matrix::new(c));

        let a = vec![vec![1, 2]];
        let b = vec![vec![2, -1], vec![-3, 4]];
        let c = vec![vec![-4, 7]];
        assert_eq!(Matrix::new(a) * Matrix::new(b), Matrix::new(c));

        let a = vec![vec![1, 2], vec![0, 3]];
        let b = vec![vec![1, 0, 2], vec![2, 1, 3]];
        let c = vec![vec![5, 2, 8], vec![6, 3, 9]];
        assert_eq!(Matrix::new(a) * Matrix::new(b), Matrix::new(c));

        let a = vec![vec![2, -1], vec![-3, 4]];
        let b = vec![vec![1], vec![2]];
        let c = vec![vec![0], vec![5]];
        assert_eq!(Matrix::new(a) * Matrix::new(b), Matrix::new(c));

        let a = vec![vec![1., -1.], vec![-2., 3.]];
        let b = vec![vec![1., 2.], vec![3., 4.]];
        let c = vec![vec![-2., -2.], vec![7., 8.]];
        assert_eq!(Matrix::new(a) * Matrix::new(b), Matrix::new(c));
    }

    #[test]
    fn test_mul_vec() {
        let a = vec![vec![-3, 4, 5], vec![1, 6, 7], vec![2, 8, 9]];
        let b = vec![-2, 0, 1];
        let c = vec![11, 5, 5];
        assert_eq!(Matrix::new(a).mul_vec(b), c);
    }

    #[test]
    fn test_identity() {}
}
