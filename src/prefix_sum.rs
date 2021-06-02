pub struct PrefixSum2D {
    sum: Vec<Vec<i64>>,
    vec: Vec<Vec<i64>>,
    h: usize,
    w: usize,
}

impl PrefixSum2D {
    pub fn new(h: usize, w: usize) -> Self {
        Self {
            sum: vec![vec![0i64; w + 1]; h + 1],
            vec: vec![vec![0i64; w]; h],
            h,
            w,
        }
    }

    pub fn add(&mut self, x: usize, y: usize, element: i64) {
        self.vec[y][x] += element;
    }

    pub fn build(&mut self) {
        for y in 0..self.h {
            for x in 0..self.w {
                self.sum[y + 1][x + 1] =
                    self.sum[y + 1][x] + self.sum[y][x + 1] - self.sum[y][x] + self.vec[y][x];
            }
        }
    }

    pub fn query(&self, x1: usize, x2: usize, y1: usize, y2: usize) -> i64 {
        self.sum[y2][x2] - self.sum[y1][x2] - self.sum[y2][x1] + self.sum[y1][x1]
    }
}

#[cfg(test)]
mod tests {
    use super::PrefixSum2D;
    #[test]
    fn test_prefix_sum_2d() {
        let mut r = PrefixSum2D::new(5, 5);
        let vec = vec![
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3, 4, 5],
        ];

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
                r.add(x, y, vec[y][x]);
            }
        }
        r.build();

        for y1 in 0..5 {
            for y2 in y1 + 1..=5 {
                for x1 in 0..5 {
                    for x2 in x1 + 1..=5 {
                        assert_eq!(r.query(x1, x2, y1, y2), f(x1, x2, y1, y2, &vec));
                    }
                }
            }
        }
    }
}
