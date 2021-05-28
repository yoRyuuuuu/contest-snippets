use cargo_snippet::snippet;

#[snippet("BinarySearchExt")]
pub trait BinarySearchExt<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

#[snippet("BinarySearchExt")]
impl<T: Ord> BinarySearchExt<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        use std::cmp::Ordering;
        let mut ng = -1i64;
        let mut ok = self.len() as i64;
        while ok - ng > 1 {
            let mid = (ok + ng) as usize / 2;
            match self[mid].cmp(x) {
                Ordering::Greater | Ordering::Equal => {
                    ok = mid as i64;
                }
                _ => ng = mid as i64,
            }
        }
        ok as usize
    }

    fn upper_bound(&self, x: &T) -> usize {
        use std::cmp::Ordering;
        let mut ng = -1i64;
        let mut ok = self.len() as i64;
        while ok - ng > 1 {
            let mid = (ok + ng) as usize / 2;
            match self[mid].cmp(x) {
                Ordering::Greater => {
                    ok = mid as i64;
                }
                _ => {
                    ng = mid as i64;
                }
            }
        }
        ok as usize
    }
}

#[snippet("BinarySearch")]
pub struct BinarySearch<F> {
    f: F,
    ok: i64,
    ng: i64,
}

#[snippet("BinarySearch")]
impl<F> BinarySearch<F>
where
    F: FnMut(i64) -> bool,
{
    pub fn search(&mut self) -> i64 {
        let mut ok = self.ok;
        let mut ng = self.ng;
        while (ok - ng).abs() > 1 {
            let mid = (ng + ok) / 2;
            if (self.f)(mid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    }
}

#[test]
fn test_binary_search() {
    let xs = vec![1, 2, 2, 2, 2, 2, 3, 4, 5];
    assert_eq!(xs.lower_bound(&2), 1);
    assert_eq!(xs.upper_bound(&2), 6);
    assert_eq!(xs.lower_bound(&8), xs.len());
    let mut bs = BinarySearch {
        f: |i: i64| xs[i as usize] >= 2,
        ok: xs.len() as i64,
        ng: -1,
    };
    assert_eq!(bs.search(), 1);
    let mut bs = BinarySearch {
        f: |i: i64| xs[i as usize] > 2,
        ok: xs.len() as i64,
        ng: -1,
    };
    assert_eq!(bs.search(), 6);
}
