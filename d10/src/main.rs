fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn to_vector(&self) -> (isize, isize) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }

    fn from_vector(vector: (isize, isize)) -> Self {
        match vector {
            (-1, 0) => Direction::Left,
            (1, 0) => Direction::Right,
            (0, -1) => Direction::Up,
            (0, 1) => Direction::Down,
            _ => panic!("Invalid vector: {:?}", vector),
        }
    }
}

fn next_coordinate(pipe: char, entry_direction: (isize, isize)) -> (isize, isize) {
    let entry_direction = Direction::from_vector(entry_direction);
    match pipe {
        // (x, y)
        'F' => match entry_direction {
            Direction::Left => Direction::Down, // from the right
            Direction::Up => Direction::Right,  // from the bottom
            _ => panic!(
                "Invalid entry direction for {}: {:?}",
                pipe, entry_direction
            ),
        },
        'J' => match entry_direction {
            Direction::Right => Direction::Up,  // from the left
            Direction::Down => Direction::Left, // from the top
            _ => panic!(
                "Invalid entry direction for {}: {:?}",
                pipe, entry_direction
            ),
        },
        'L' => match entry_direction {
            Direction::Left => Direction::Up,    // from the right
            Direction::Down => Direction::Right, // from the top
            _ => panic!(
                "Invalid entry direction for {}: {:?}",
                pipe, entry_direction
            ),
        },
        '7' => match entry_direction {
            Direction::Right => Direction::Down, // from the left
            Direction::Up => Direction::Left,    // from the bottom
            _ => panic!(
                "Invalid entry direction for {}: {:?}",
                pipe, entry_direction
            ),
        },
        '-' => match entry_direction {
            Direction::Left => Direction::Left,   // from the left
            Direction::Right => Direction::Right, // from the bottom
            _ => panic!(
                "Invalid entry direction for {}: {:?}",
                pipe, entry_direction
            ),
        },
        '|' => match entry_direction {
            Direction::Up => Direction::Up,     // from the left
            Direction::Down => Direction::Down, // from the bottom
            _ => panic!(
                "Invalid entry direction for {}: {:?}",
                pipe, entry_direction
            ),
        },
        _ => panic!("Invalid pipe: {}", pipe),
    }
    .to_vector()
}

fn part1(input: &str) -> usize {
    let grid = parse(input);
    let mut x: usize = 0; // current x position
    let mut y: usize = 0; // current y position

    // find the starting position
    'outer: for (i, row) in grid.iter().enumerate() {
        for (j, pipe) in row.iter().enumerate() {
            if *pipe == 'S' {
                x = j;
                y = i;
                break 'outer;
            }
        }
    }

    x += 1; // start to the right of 'S'
    let mut distance = 1;
    let mut direction = Direction::Right.to_vector();
    while grid[y][x] != 'S' {
        direction = next_coordinate(grid[y][x], direction);
        (x, y) = (
            (x as isize + direction.0) as usize,
            (y as isize + direction.1) as usize,
        );
        distance += 1;
    }
    distance / 2
}

#[cfg(test)]
mod test {
    use super::*;
    static EXAMPLE: &str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 8);
    }
}
