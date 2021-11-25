use std::cmp::Ordering;

type Link<T> = Option<Box<Node<T>>>;
#[derive(Clone, Debug)]
struct Node<T> {
    element: T,
    priority: i64,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> Node<T> {
    fn new(element: T, priority: i64) -> Self {
        Self {
            element,
            priority,
            left: None,
            right: None,
            len: 1,
        }
    }
}

impl<T> Node<T> {
    pub fn len(node: &Link<T>) -> usize {
        node.as_ref().map_or(0, |node| node.len)
    }

    fn update(node: Box<Node<T>>) -> Link<T> {
        let mut node = node;
        node.len = Self::len(&node.left) + Self::len(&node.right) + 1;
        Some(node)
    }

    fn merge(left: Link<T>, right: Link<T>) -> Link<T> {
        match (left, right) {
            (None, None) => None,
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (Some(mut left), Some(mut right)) => {
                if left.priority > right.priority {
                    left.right = Self::merge(left.right, Some(right));
                    Self::update(left)
                } else {
                    right.left = Self::merge(Some(left), right.left);
                    Self::update(right)
                }
            }
        }
    }

    fn split(node: Link<T>, k: usize) -> (Link<T>, Link<T>) {
        match node {
            None => (None, None),
            Some(mut node) => {
                let left_len = Self::len(&node.left);
                if k <= left_len {
                    let (left, right) = Self::split(node.left, k);
                    node.left = right;
                    (left, Self::update(node))
                } else {
                    let (left, right) = Self::split(node.right, k - left_len - 1);
                    node.right = left;
                    (Self::update(node), right)
                }
            }
        }
    }

    pub fn lower_bound(node: &Link<T>, element: &T) -> usize
    where
        T: Ord,
    {
        match node {
            None => 0,
            Some(node) => {
                if *element <= node.element {
                    Self::lower_bound(&node.left, element)
                } else {
                    Self::len(&node.left) + Self::lower_bound(&node.right, element) + 1
                }
            }
        }
    }

    pub fn get(node: &Link<T>, k: usize) -> Option<&T> {
        match node {
            None => None,
            Some(node) => match Node::len(&node.left).cmp(&k) {
                Ordering::Equal => Some(&node.element),
                Ordering::Greater => Self::get(&node.left, k),
                Ordering::Less => Self::get(&node.right, k - Node::len(&node.left) - 1),
            },
        }
    }

    pub fn insert_at(node: Link<T>, k: usize, element: T, priority: i64) -> Link<T> {
        let (left, right) = Self::split(node, k);
        let node = Self::merge(left, Some(Box::new(Node::new(element, priority))));
        Self::merge(node, right)
    }

    pub fn erase_at(node: Link<T>, k: usize) -> Link<T> {
        let (node1, node2) = Self::split(node, k);
        let (_, node3) = Self::split(node2, 1);
        Self::merge(node1, node3)
    }

    pub fn depth(node: &Link<T>) -> usize {
        match node {
            None => 0,
            Some(node) => std::cmp::max(Self::depth(&node.left), Self::depth(&node.right)) + 1,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TreapSet<T> {
    seed: i64,
    root: Link<T>,
}

impl<T> TreapSet<T> {
    pub fn new() -> Self {
        Self {
            seed: 0xdeadc0de,
            root: None,
        }
    }

    pub fn len(&self) -> usize {
        Node::len(&self.root)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn depth(&mut self) -> usize {
        Node::depth(&self.root)
    }

    pub fn insert(&mut self, element: T)
    where
        T: Ord,
    {
        let k = Node::lower_bound(&self.root, &element);
        let root = self.root.take();
        self.root = match root {
            None => Some(Box::new(Node::new(element, self.next()))),
            Some(root) => Node::insert_at(Some(root), k, element, self.next()),
        };
    }

    pub fn remove(&mut self, element: &T) -> bool
    where
        T: Ord,
    {
        let k = Node::lower_bound(&self.root, element);
        match self.root.take() {
            Some(root) => {
                self.root = Node::erase_at(Some(root), k);
                true
            }
            None => false,
        }
    }

    pub fn lower_bound(&self, element: &T) -> usize
    where
        T: Ord,
    {
        Node::lower_bound(&self.root, element)
    }

    pub fn get(&self, k: usize) -> Option<&T> {
        Node::get(&self.root, k)
    }

    fn next(&mut self) -> i64 {
        self.seed = self
            .seed
            .wrapping_mul(0x12345678deadc0d1)
            .wrapping_add(0x1551);
        self.seed
    }
}

impl<T> Default for TreapSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    // use super::treap::Treap;
    use super::TreapSet;
    #[test]
    fn test_treap_set() {
        let mut treap = TreapSet::new();
        treap.insert(2);
        treap.insert(1);
        treap.insert(6);
        treap.insert(4);
        println!("{:?}", treap);
        assert_eq!(treap.get(0), Some(&1));
        assert_eq!(treap.get(1), Some(&2));
        assert_eq!(treap.get(2), Some(&4));
        assert_eq!(treap.get(3), Some(&6));
    }

    #[test]
    fn test_treap_set_random() {
        use rand::prelude::*;
        let mut treap = TreapSet::new();
        let n = 1000;
        let mut rng = thread_rng();
        let mut xs = (0..n)
            .map(|_| rng.gen_range(0..1_000_000_000))
            .collect::<Vec<_>>();
        for x in xs.iter() {
            treap.insert(*x);
        }
        xs.sort_unstable();

        for i in 0..n {
            assert_eq!(treap.get(i), Some(&xs[i]));
        }

        println!("depth = {}", treap.depth())
    }
}
