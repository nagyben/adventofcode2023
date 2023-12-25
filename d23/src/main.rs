use std::collections::HashMap;

use petgraph::{
    algo::all_simple_paths,
    graph::DiGraph,
    prelude::*,
    Graph,
};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn part1(input: &str) -> usize {
    let map = parse(input);
    let (graph, node_map) = create_graph_with_slopes(&map);
    get_longest_path(map, graph, node_map)
}

fn part2(input: &str) -> usize {
    let map = parse(input);
    let (graph, node_map) = create_graph_without_slopes(&map);
    get_longest_path(map, graph, node_map)
}

fn get_longest_path<N, E, Ty>(
    map: Vec<Vec<char>>,
    graph: Graph<N, E, Ty>,
    node_map: HashMap<Point, NodeIndex>,
) -> usize
where
    Ty: petgraph::EdgeType,
{
    let (start, end) = find_start_end(&map);
    let paths = all_simple_paths::<Vec<_>, _>(
        &graph,
        *node_map.get(&start).unwrap(),
        *node_map.get(&end).unwrap(),
        0,
        None,
    )
    .collect::<Vec<_>>();

    paths.iter().fold(
        0,
        |acc, path| {
            if path.len() > acc {
                path.len()
            } else {
                acc
            }
        },
    ) - 1
}

fn find_start_end(map: &[Vec<char>]) -> (Point, Point) {
    let start = Point {
        y: 0,
        x: map[0]
            .iter()
            .enumerate()
            .find(|(_, c)| **c == '.')
            .unwrap()
            .0,
    };
    let end = Point {
        y: map.len() - 1,
        x: map[map.len() - 1]
            .iter()
            .enumerate()
            .find(|(_, c)| **c == '.')
            .unwrap()
            .0,
    };
    (start, end)
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn create_graph_with_slopes(
    map: &[Vec<char>],
) -> (DiGraph<Point, isize>, HashMap<Point, NodeIndex>) {
    let mut graph: Graph<Point, isize> = DiGraph::new();
    let mut node_map: HashMap<Point, NodeIndex> = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '#' {
                continue;
            }
            let point = Point { x, y };
            node_map.insert(point, graph.add_node(point));
        }
    }

    // Add edges
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            match *c {
                '#' => continue,
                '.' => {
                    let point = Point { x, y };
                    // check up
                    if y > 0 && map[y - 1][x] != '#' && map[y - 1][x] != 'v' {
                        graph.add_edge(node_map[&point], node_map[&Point { x, y: y - 1 }], -1);
                    }
                    // check down
                    if y < map.len() - 1 && map[y + 1][x] != '#' && map[y + 1][x] != '^' {
                        graph.add_edge(node_map[&point], node_map[&Point { x, y: y + 1 }], -1);
                    }
                    // check left
                    if x > 0 && map[y][x - 1] != '#' && map[y][x - 1] != '>' {
                        graph.add_edge(node_map[&point], node_map[&Point { x: x - 1, y }], -1);
                    }
                    // check right
                    if x < row.len() - 1 && map[y][x + 1] != '#' && map[y][x + 1] != '<' {
                        graph.add_edge(node_map[&point], node_map[&Point { x: x + 1, y }], -1);
                    }
                }
                '^' => {
                    let point = Point { x, y };
                    // check up
                    if y > 0 && map[y - 1][x] != '#' && map[y - 1][x] != 'v' {
                        graph.add_edge(node_map[&point], node_map[&Point { x, y: y - 1 }], -1);
                    }
                }
                'v' => {
                    let point = Point { x, y };
                    // check down
                    if y < map.len() - 1 && map[y + 1][x] != '#' && map[y + 1][x] != '^' {
                        graph.add_edge(node_map[&point], node_map[&Point { x, y: y + 1 }], -1);
                    }
                }
                '>' => {
                    let point = Point { x, y };
                    // check right
                    if x < row.len() - 1 && map[y][x + 1] != '#' && map[y][x + 1] != '<' {
                        graph.add_edge(node_map[&point], node_map[&Point { x: x + 1, y }], -1);
                    }
                }
                '<' => {
                    let point = Point { x, y };
                    // check left
                    if x > 0 && map[y][x - 1] != '#' && map[y][x - 1] != '>' {
                        graph.add_edge(node_map[&point], node_map[&Point { x: x - 1, y }], -1);
                    }
                }
                _ => unreachable!("Invalid character: {}", c),
            }
        }
    }
    (graph, node_map)
}

fn create_graph_without_slopes(
    map: &[Vec<char>],
) -> (UnGraph<Point, isize>, HashMap<Point, NodeIndex>) {
    let mut graph: UnGraph<Point, isize> = Graph::new_undirected();
    let mut node_map: HashMap<Point, NodeIndex> = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '#' {
                continue;
            }
            let point = Point { x, y };
            node_map.insert(point, graph.add_node(point));
        }
    }

    // Add edges
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            match *c {
                '#' => continue,
                _ => {
                    let cur_node_index = node_map[&Point { x, y }];
                    // check up
                    if y > 0 && map[y - 1][x] != '#' {
                        let target_node_index = node_map[&Point { x, y: y - 1 }];
                        if !graph.contains_edge(cur_node_index, target_node_index)
                            && !graph.contains_edge(target_node_index, cur_node_index)
                        {
                            graph.add_edge(cur_node_index, target_node_index, -1);
                        }
                    }
                    // check down
                    if y < map.len() - 1 && map[y + 1][x] != '#' {
                        let target_node_index = node_map[&Point { x, y: y + 1 }];
                        if !graph.contains_edge(cur_node_index, target_node_index)
                            && !graph.contains_edge(target_node_index, cur_node_index)
                        {
                            graph.add_edge(cur_node_index, target_node_index, -1);
                        }
                    }
                    // check left
                    if x > 0 && map[y][x - 1] != '#' {
                        let target_node_index = node_map[&Point { x: x - 1, y }];
                        if !graph.contains_edge(cur_node_index, target_node_index)
                            && !graph.contains_edge(target_node_index, cur_node_index)
                        {
                            graph.add_edge(cur_node_index, target_node_index, -1);
                        }
                    }
                    // check right
                    if x < row.len() - 1 && map[y][x + 1] != '#' {
                        let target_node_index = node_map[&Point { x: x + 1, y }];
                        if !graph.contains_edge(cur_node_index, target_node_index)
                            && !graph.contains_edge(target_node_index, cur_node_index)
                        {
                            graph.add_edge(cur_node_index, target_node_index, -1);
                        }
                    }
                }
            }
        }
    }
    (graph, node_map)
}

fn dot<T>(graph: Graph<T, isize>, filename: &str)
where
    T: std::fmt::Debug,
{
    use petgraph::dot;
    use std::fs;
    let dot_txt = format!(
        "{:?}",
        dot::Dot::with_config(&graph, &[dot::Config::EdgeNoLabel])
    );
    fs::write(filename, dot_txt).expect("Unable to write to file");
}

fn dot_undirected<T>(graph: UnGraph<T, isize>, filename: &str)
where
    T: std::fmt::Debug,
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
    use petgraph::algo::condensation;

    use super::*;

    static EXAMPLE: &str = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 94);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 154);
    }

    #[test]
    fn test_dot_part1() {
        let map = parse(EXAMPLE);
        let (graph, _) = create_graph_with_slopes(&map);
        dot(graph, "part1.dot");
    }

    #[test]
    fn test_dot_part2() {
        let map = parse(EXAMPLE);
        let (graph, _) = create_graph_without_slopes(&map);
        dot_undirected(graph, "part2.dot");
    }

    #[test]
    fn test_condense() {
        let map = parse(EXAMPLE);
        let (graph, _) = create_graph_without_slopes(&map);
        let condensed_graph = condensation(graph, false);
        dot_undirected(condensed_graph, "condensed_graph.dot");
    }
}
