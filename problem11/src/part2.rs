use common::count_paths;
use macros::embed_split_strip_colon_lines;
use petgraph::graph::DiGraph;
use std::collections::BTreeMap;

embed_split_strip_colon_lines!("../input.txt");

pub fn main() {
    let mut graph = DiGraph::<&'static str, i32>::new();
    let mut indices = BTreeMap::new();

    indices.insert("out", graph.add_node("out"));

    for (name, _) in INPUT {
        indices.insert(name, graph.add_node(name));
    }

    let edges = INPUT
        .into_iter()
        .flat_map(|(src, dst)| dst.into_iter().map(move |dst| (src, *dst)))
        .map(|(src, dst)| (indices[src], indices[dst]));

    graph.extend_with_edges(edges);

    let res = count_paths(
        &graph,
        indices["svr"],
        indices["out"],
        indices["dac"],
        indices["fft"],
    );

    std::hint::black_box(res);

    #[cfg(feature = "cli")]
    println!("Paths: {res}");
}
