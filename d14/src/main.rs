use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

enum Direction {
    North,
    East,
    South,
    West,
}

fn part1(input: &str) -> usize {
    let mut rocks = parse(input);
    tilt(&mut rocks, Direction::North);
    calculate_load(&rocks)
}

fn calculate_load(rocks: &Vec<Vec<char>>) -> usize {
    rocks.iter().rev().enumerate().fold(0, |acc, (y, row)| {
        acc + row.iter().enumerate().fold(
            0,
            |acc, (_, c)| {
                if *c == 'O' {
                    acc + y + 1
                } else {
                    acc
                }
            },
        )
    })
}

fn part2(input: &str) -> usize {
    let mut rocks = parse(input);

    // store the state of the rocks and its cycle number
    let mut cache: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    cache.insert(rocks.clone(), 0);

    // we won't actually loop a billion times
    // we will break out once we have found the repeating pattern
    for i in 0..1_000_000_000 {
        cycle(&mut rocks);
        if let Some(cache_i) = cache.get(&rocks) {
            // we found the repeating pattern
            let cycles = i + 1 - cache_i;

            // now we only need to check the remainder cycles
            // to know where we will end up
            let remaining = (1_000_000_000 - i - 1) % cycles;
            for _ in 0..remaining {
                cycle(&mut rocks);
            }
            break;
        }
        cache.insert(rocks.clone(), i + 1);
    }
    calculate_load(&rocks)
}

fn tilt(rocks: &mut Vec<Vec<char>>, direction: Direction) -> &Vec<Vec<char>> {
    let mut movement = true;
    while movement {
        movement = false;
        for y in 0..rocks.len() {
            for x in 0..rocks[y].len() {
                match direction {
                    Direction::North => {
                        if y > 0 {
                            if rocks[y][x] == 'O' && rocks[y - 1][x] == '.' {
                                rocks[y - 1][x] = 'O';
                                rocks[y][x] = '.';
                                movement = true;
                            }
                        }
                    }
                    Direction::East => {
                        if x < rocks[y].len() - 1 {
                            if rocks[y][x] == 'O' && rocks[y][x + 1] == '.' {
                                rocks[y][x + 1] = 'O';
                                rocks[y][x] = '.';
                                movement = true;
                            }
                        }
                    }
                    Direction::South => {
                        if y < rocks.len() - 1 {
                            if rocks[y][x] == 'O' && rocks[y + 1][x] == '.' {
                                rocks[y + 1][x] = 'O';
                                rocks[y][x] = '.';
                                movement = true;
                            }
                        }
                    }
                    Direction::West => {
                        if x > 0 {
                            if rocks[y][x] == 'O' && rocks[y][x - 1] == '.' {
                                rocks[y][x - 1] = 'O';
                                rocks[y][x] = '.';
                                movement = true;
                            }
                        }
                    }
                }
            }
        }
    }
    rocks
}

fn cycle(rocks: &mut Vec<Vec<char>>) {
    tilt(rocks, Direction::North);
    tilt(rocks, Direction::West);
    tilt(rocks, Direction::South);
    tilt(rocks, Direction::East);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = r#"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."#;

    static TILTED1: &str = r#"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."#;

    static CYCLE1: &str = r#".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."#;

    static CYCLE2: &str = r#".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"#;
    static CYCLE3: &str = r#".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE1), 136);
    }

    #[test]
    fn test_tilt() {
        let mut rocks = parse(EXAMPLE1);
        let expected = parse(TILTED1);
        tilt(&mut rocks, Direction::North);
        assert_eq!(*rocks, expected);
    }

    #[test]
    fn test_cycle() {
        let mut rocks = parse(EXAMPLE1);
        let expected = parse(CYCLE1);
        cycle(&mut rocks);
        assert_eq!(*rocks, expected);
        let expected = parse(CYCLE2);
        cycle(&mut rocks);
        assert_eq!(*rocks, expected);
        let expected = parse(CYCLE3);
        cycle(&mut rocks);
        assert_eq!(*rocks, expected);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE1), 64);
    }
}
