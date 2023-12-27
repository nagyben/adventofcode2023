use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use petgraph::{graph::UnGraph, EdgeType, Graph};
use rustworkx_core::connectivity::stoer_wagner_min_cut;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
}

fn parse(input: &str) -> IResult<&str, HashMap<&str, Vec<&str>>> {
    let (input, map) = separated_list1(
        line_ending,
        separated_pair(alpha1, tag(": "), separated_list1(tag(" "), alpha1)),
    )(input)?;
    Ok((input, map.iter().cloned().collect::<HashMap<_, _>>()))
}

fn part1(input: &str) -> usize {
    let map = parse(input).unwrap().1;
    let graph = create_graph(&map);
    type Error = Box<dyn std::error::Error>;
    let min_cut: Result<Option<(i32, Vec<petgraph::prelude::NodeIndex>)>, Error> =
        stoer_wagner_min_cut(&graph, |_| Ok(1));
    let subgraph_nodes = min_cut.unwrap().unwrap().1.len();
    (graph.node_count() - subgraph_nodes) * subgraph_nodes
}

fn create_graph<'a>(map: &'a HashMap<&'a str, Vec<&'a str>>) -> UnGraph<&'a str, ()> {
    let mut graph = Graph::new_undirected();
    let mut nodes = HashMap::new();
    for (key, values) in map {
        if !nodes.contains_key(key) {
            nodes.insert(*key, graph.add_node(*key));
        }
        for value in values {
            if !nodes.contains_key(value) {
                nodes.insert(*value, graph.add_node(*value));
            }
        }
    }
    for (key, values) in map {
        for value in values {
            graph.add_edge(nodes[key], nodes[value], ());
        }
    }
    graph
}

fn dot<N, E, Ty>(graph: &Graph<N, E, Ty>, filename: &str)
where
    N: std::fmt::Debug,
    E: std::fmt::Debug,
    Ty: EdgeType,
{
    use petgraph::dot;
    use std::fs;
    let dot_txt = format!(
        "{:?}",
        dot::Dot::with_config(&graph, &[dot::Config::EdgeNoLabel])
    );
    fs::write(filename, dot_txt).expect("Unable to write to file");
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"jqt: rhn xhk nvd
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
frs: qnr lhk lsr"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 54);
    }
}
