use cargo_snippet::snippet;

#[snippet("UnionFind")]
pub struct UnionFind {
    size: Vec<usize>,
    par: Vec<usize>,
}

#[snippet("UnionFind")]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        let mut uf = Self {
            size: vec![0; n],
            par: vec![0; n],
        };

        for i in 0..n {
            uf.par[i] = i;
            uf.size[i] = 1;
        }

        uf
    }

    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        return self.find_root(x) == self.find_root(y);
    }

    pub fn find_root(&mut self, x: usize) -> usize {
        if x != self.par[x] {
            self.par[x] = self.find_root(self.par[x]);
        }
        return self.par[x];
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let x = self.find_root(x);
        let y = self.find_root(y);
        if x == y {
            return false;
        }
        if self.size[x] > self.size[y] {
            self.par[y] = x;
            self.size[x] += self.size[y];
        } else {
            self.par[x] = y;
            self.size[y] += self.size[x];
        }
        return true;
    }

    pub fn size(&mut self, x: usize) -> usize {
        let x = self.find_root(x);
        return self.size[x];
    }
}

#[cfg(test)]
mod tests {
    use super::UnionFind;

    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::new(5);
        uf.unite(0, 1);

        assert_eq!(uf.is_same(0, 1), true);
        assert_eq!(uf.size(0), 2);

        uf.unite(2, 4);

        assert_eq!(uf.is_same(2, 4), true);
        assert_eq!(uf.size(2), 2);

        uf.unite(0, 2);

        assert_eq!(uf.is_same(0, 2), true);
        assert_eq!(uf.is_same(0, 4), true);
        assert_eq!(uf.is_same(1, 2), true);
        assert_eq!(uf.size(0), 4);
    }
}
