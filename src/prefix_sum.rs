use crate::number::Number;
use cargo_snippet::snippet;
#[snippet("PrefixSum2D")]
use std::ops::{Add, Sub};

#[snippet("PrefixSum2D")]
pub struct PrefixSum2D<T> {
    sum: Vec<Vec<T>>,
    vec: Vec<Vec<T>>,
    h: usize,
    w: usize,
}

#[snippet("PrefixSum2D")]
impl<T> From<Vec<Vec<T>>> for PrefixSum2D<T>
where
    T: Number + Clone + Copy + Add<Output = T> + Sub<Output = T>,
{
    fn from(vec: Vec<Vec<T>>) -> Self {
        let h = vec.len();
        let w = vec[0].len();
        let mut slf = Self {
            sum: vec![vec![T::zero(); w + 1]; h + 1],
            vec,
            h,
            w,
        };
        slf.build();

        return slf;
    }
}

#[snippet("PrefixSum2D")]
impl<T> PrefixSum2D<T>
where
    T: Number + Clone + Copy + Add<Output = T> + Sub<Output = T>,
{
    pub fn new(h: usize, w: usize) -> Self {
        Self {
            sum: vec![vec![T::zero(); w + 1]; h + 1],
            vec: vec![vec![T::zero(); w]; h],
            h,
            w,
        }
    }

    pub fn add(&mut self, x: usize, y: usize, element: T) {
        self.vec[y][x] = self.vec[y][x] + element;
    }

    pub fn build(&mut self) {
        for y in 0..self.h {
            for x in 0..self.w {
                self.sum[y + 1][x + 1] =
                    self.sum[y + 1][x] + self.sum[y][x + 1] - self.sum[y][x] + self.vec[y][x];
            }
        }
    }

    pub fn query(&self, x1: usize, x2: usize, y1: usize, y2: usize) -> T {
        self.sum[y2][x2] - self.sum[y1][x2] - self.sum[y2][x1] + self.sum[y1][x1]
    }
}

#[cfg(test)]
mod tests {
    use super::PrefixSum2D;
    #[test]
    fn test_prefix_sum_2d() {
        let vec = vec![
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3, 4, 5],
        ];

        let mut r1 = PrefixSum2D::new(5, 5);
        let r2 = PrefixSum2D::from(vec.clone());

        let f = |x1: usize, x2: usize, y1: usize, y2: usize, vec: &Vec<Vec<i64>>| {
            let mut res = 0;
            for y in y1..y2 {
                for x in x1..x2 {
                    res += vec[y][x];
                }
            }

            return res;
        };

        for y in 0..5 {
            for x in 0..5 {
                r1.add(x, y, vec[y][x]);
            }
        }
        r1.build();

        for y1 in 0..5 {
            for y2 in y1 + 1..=5 {
                for x1 in 0..5 {
                    for x2 in x1 + 1..=5 {
                        assert_eq!(r1.query(x1, x2, y1, y2), f(x1, x2, y1, y2, &vec));
                        assert_eq!(r2.query(x1, x2, y1, y2), f(x1, x2, y1, y2, &vec));
                    }
                }
            }
        }
    }
}
