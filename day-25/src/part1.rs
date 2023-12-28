use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::graph::UnGraph;
use rustworkx_core::Result;
use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    let mut g = UnGraph::new_undirected();
    let mut node_indices = HashMap::new();

    input.lines().for_each(|line| {
        let (from, tos) = line.split_once(": ").unwrap();
        let from_index = *node_indices.entry(from).or_insert_with(|| g.add_node(from));
        tos.split(' ').for_each(|to| {
            let to_index = *node_indices.entry(to).or_insert_with(|| g.add_node(to));
            g.add_edge(from_index, to_index, ());
        });
    });

    let min_cut_res: Result<Option<(usize, Vec<_>)>> = stoer_wagner_min_cut(&g, |_| Ok(1));
    let (min_cut, partition) = min_cut_res.unwrap().unwrap();
    assert_eq!(min_cut, 3);
    partition.len() * (g.node_count() - partition.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    static EXAMPLE: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[rstest]
    #[case(EXAMPLE, 54)]
    fn test_example(#[case] input: &str, #[case] expected: usize) {
        tracing_init();
        assert_eq!(solve(input), expected);
    }
}
