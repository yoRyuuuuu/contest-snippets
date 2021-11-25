use std::cmp::Ordering;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Edge {
    to: usize,
    cost: i64,
}

impl Edge {
    pub fn new(to: usize, cost: i64) -> Self {
        Self { to, cost }
    }
}

pub fn dijkstra(graph: &[Vec<Edge>], start: usize) -> Vec<i64> {
    use std::collections::BinaryHeap;
    let mut dist: Vec<_> = (0..graph.len()).map(|_| std::i64::MAX).collect();
    let mut heap = BinaryHeap::new();
    dist[start] = 0;
    heap.push(State {
        cost: 0,
        cur_node: start,
    });

    while let Some(State { cost, cur_node }) = heap.pop() {
        if cost > dist[cur_node] {
            continue;
        }

        for edge in &graph[cur_node] {
            let next = State {
                cost: cost + edge.cost,
                cur_node: edge.to,
            };

            if next.cost < dist[next.cur_node] {
                heap.push(next);
                dist[next.cur_node] = next.cost;
            }
        }
    }

    dist
}
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct State {
    cost: i64,
    cur_node: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.cur_node.cmp(&other.cur_node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(test)]
mod tests {
    use super::{dijkstra, Edge};

    #[test]
    fn dijkstra_test() {
        let mut graph = vec![vec![]; 5];
        let data = vec![(0, 1, 5), (0, 3, 3), (0, 2, 1), (1, 2, 2), (3, 4, 1)];

        for d in data {
            graph[d.0].push(Edge::new(d.1, d.2));
            graph[d.1].push(Edge::new(d.0, d.2));
        }

        let dist = dijkstra(&graph, 0);
        let acc = vec![0, 3, 1, 3, 4];
        for i in 0..5 {
            assert_eq!(dist[i], acc[i]);
        }
    }
}
