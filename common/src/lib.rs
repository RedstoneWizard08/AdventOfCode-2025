#![feature(const_default, const_trait_impl)]

use petgraph::{
    Direction,
    graph::{DiGraph, NodeIndex},
    visit::Topo,
};

pub struct MultiCursor<const W: usize, const H: usize, T: Copy> {
    data: [[T; W]; H],
    cursor: usize,
    default: T,
}

impl<const W: usize, const H: usize, T: Copy> MultiCursor<W, H, T> {
    pub const fn new(data: [[T; W]; H]) -> Self
    where
        T: [const] Default,
    {
        Self {
            data,
            cursor: 0,
            default: T::default(),
        }
    }

    pub fn next(&mut self) -> Option<[T; H]> {
        if self.cursor < W {
            let mut res = [self.default; H];

            for i in 0..H {
                res[i] = self.data[i][self.cursor];
            }

            self.cursor += 1;

            Some(res)
        } else {
            None
        }
    }
}

pub trait Squared {
    type Output;

    fn sqr(&self) -> Self::Output;
}

impl Squared for f32 {
    type Output = f32;

    #[inline(always)]
    fn sqr(&self) -> Self::Output {
        self * self
    }
}

// This utter SORCERY had me reading tons of stack overflow posts AND blogs on how DAGs work.
// Fuck this shit :(

pub fn count_paths_dag(graph: &DiGraph<&str, i32>, start: NodeIndex, end: NodeIndex) -> u128 {
    let mut topo = Topo::new(&graph);
    let mut indices = Vec::new();

    while let Some(idx) = topo.next(graph) {
        indices.push(idx);
    }

    let mut index = vec![0u128; graph.node_count()];

    index[start.index()] = 1;

    for idx in &indices {
        let ways = index[idx.index()];

        if ways == 0 {
            // duh
            continue;
        }

        for neighbor in graph.neighbors_directed(*idx, Direction::Outgoing) {
            index[neighbor.index()] += ways;
        }
    }

    index[end.index()]
}

pub fn count_paths(
    graph: &DiGraph<&str, i32>,
    a: NodeIndex,
    b: NodeIndex,
    dac: NodeIndex,
    fft: NodeIndex,
) -> u128 {
    // 'twas the only way it didn't hang
    // so, whatever.

    let p1 = count_paths_dag(graph, a, dac);
    let p2 = count_paths_dag(graph, dac, fft);
    let p3 = count_paths_dag(graph, fft, b);

    let q1 = count_paths_dag(graph, a, fft);
    let q2 = count_paths_dag(graph, fft, dac);
    let q3 = count_paths_dag(graph, dac, b);

    // combos my beloved
    p1 * p2 * p3 + q1 * q2 * q3
}
