fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    universe_distances(input, 2)
}

fn part2(input: &str) -> usize {
    universe_distances(input, 1000000)
}

fn universe_distances(input: &str, expansion_factor: usize) -> usize {
    let universe = parse(input);
    let expanded_universe = expand(&universe, expansion_factor);
    expanded_universe
        .iter()
        .map(|g| {
            expanded_universe
                .iter()
                .filter(|g2| g != *g2)
                .map(|g2| distance(g, g2))
                .sum::<usize>()
        })
        .sum::<usize>()
        / 2 //divide by two because we're double counting
}

#[derive(Debug, PartialEq, Clone)]
struct Galaxy {
    x: usize,
    y: usize,
}

type Universe = Vec<Galaxy>;

fn expand(universe: &Universe, expansion_factor: usize) -> Universe {
    // find the rows and columns that don't have any galaxies
    let mut expanded_universe: Universe = universe.clone();
    let empty_rows = universe
        .iter()
        .enumerate()
        .filter(|(i, _)| is_row_empty(universe, *i))
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    let empty_columns = universe
        .iter()
        .enumerate()
        .filter(|(i, _)| is_column_empty(universe, *i))
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    for row in empty_rows.iter().rev() {
        expanded_universe
            .iter_mut()
            .filter(|g| g.y > *row)
            .for_each(|g| {
                g.y += expansion_factor - 1;
            });
    }

    for col in empty_columns.iter().rev() {
        expanded_universe
            .iter_mut()
            .filter(|g| g.x > *col)
            .for_each(|g| {
                g.x += expansion_factor - 1;
            });
    }

    expanded_universe
}

fn is_row_empty(universe: &Universe, row: usize) -> bool {
    universe.iter().find(|g| g.y == row).is_none()
}

fn is_column_empty(universe: &Universe, column: usize) -> bool {
    universe.iter().find(|g| g.x == column).is_none()
}

fn parse(input: &str) -> Universe {
    let mut result = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                result.push(Galaxy { x, y });
            }
        }
    }
    result
}

fn distance(a: &Galaxy, b: &Galaxy) -> usize {
    (a.x as isize - b.x as isize).abs() as usize + (a.y as isize - b.y as isize).abs() as usize
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

    #[test]
    fn test_parse() {
        let expected = vec![
            Galaxy { x: 3, y: 0 },
            Galaxy { x: 7, y: 1 },
            Galaxy { x: 0, y: 2 },
            Galaxy { x: 6, y: 4 },
            Galaxy { x: 1, y: 5 },
            Galaxy { x: 9, y: 6 },
            Galaxy { x: 7, y: 8 },
            Galaxy { x: 0, y: 9 },
            Galaxy { x: 4, y: 9 },
        ];
        let actual = parse(EXAMPLE);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_expand() {
        static EXAMPLE_AFTER_EXPANSION: &str = r#"....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#......."#;
        let input = parse(EXAMPLE);
        let expected = parse(EXAMPLE_AFTER_EXPANSION);
        assert_eq!(expected, expand(&input, 2));
    }

    #[test]
    fn test_distance() {
        let a = Galaxy { x: 4, y: 0 };
        let b = Galaxy { x: 9, y: 10 };
        assert_eq!(15, distance(&a, &b));
    }

    #[test]
    fn test_universe_distances() {
        assert_eq!(374, universe_distances(EXAMPLE, 2));
        assert_eq!(1030, universe_distances(EXAMPLE, 10));
        assert_eq!(8410, universe_distances(EXAMPLE, 100));
    }
}
