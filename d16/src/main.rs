use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Ray {
    x: usize,
    y: usize,
    direction: Direction,
}

fn next_coordinate(tile: char, entry_direction: Direction) -> (isize, isize) {
    match tile {
        // (x, y)
        '/' => match entry_direction {
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Left,
        },
        '\\' => match entry_direction {
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Right,
        },
        '-' => match entry_direction {
            Direction::Left => Direction::Left,
            Direction::Right => Direction::Right,
            // if up or down just go right and we'll initialize a new ray going left in the queue
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Right,
        },
        '|' => match entry_direction {
            Direction::Up => Direction::Up,
            Direction::Down => Direction::Down,
            // if left or right just go up and we'll initialize a new ray going down
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Up,
        },
        '.' => entry_direction,
        _ => unreachable!("Invalid tile: {}", tile),
    }
    .to_vector()
}

fn split_ray(ray: &Ray, grid: &Vec<Vec<char>>) -> Option<Ray> {
    let tile = grid[ray.y][ray.x];

    match tile {
        '-' => match ray.direction {
            Direction::Up | Direction::Down => Some(Ray {
                x: ray.x,
                y: ray.y,
                direction: Direction::Left,
            }),
            _ => None,
        },
        '|' => match ray.direction {
            Direction::Left | Direction::Right => Some(Ray {
                x: ray.x,
                y: ray.y,
                direction: Direction::Down,
            }),
            _ => None,
        },
        _ => None,
    }
}

fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    raycast(
        &grid,
        Ray {
            x: 0,
            y: 0,
            direction: Direction::Right,
        },
    )
}

fn raycast(grid: &Vec<Vec<char>>, initial_ray: Ray) -> usize {
    let mut queue: VecDeque<Ray> = VecDeque::new();
    let mut seen_rays: HashSet<Ray> = HashSet::new();
    let mut energized: Vec<Vec<char>> = vec![vec!['.'; grid[0].len()]; grid.len()];
    queue.push_back(initial_ray);
    while let Some(mut ray) = queue.pop_front() {
        loop {
            energized[ray.y][ray.x] = '#';
            // if grid[ray.y][ray.x] != '.' {
            //     energized[ray.y][ray.x] = grid[ray.y][ray.x];
            // }
            if let Some(new_ray) = split_ray(&ray, &grid) {
                queue.push_back(new_ray);
            }
            // break if we've already seen this ray position and direction to avoid infinite loops
            if seen_rays.contains(&ray) {
                break;
            }
            seen_rays.insert(ray);
            let direction = next_coordinate(grid[ray.y][ray.x], ray.direction);
            let (new_x, new_y) = (
                (ray.x as isize + direction.0),
                (ray.y as isize + direction.1),
            );
            ray.direction = Direction::from_vector(direction);
            // break if the ray is out of bounds
            if new_x as usize >= grid[0].len()
                || new_y as usize >= grid.len()
                || new_x < 0
                || new_y < 0
            {
                break;
            }
            ray.x = new_x as usize;
            ray.y = new_y as usize;
        }
    }

    energized.iter().fold(0, |acc, row| {
        acc + row.iter().filter(|&c| *c == '#').count()
    })
}

fn part2(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut initial_rays: Vec<Ray> = vec![];
    for y in 0..grid.len() {
        initial_rays.push(Ray {
            x: 0,
            y,
            direction: Direction::Right,
        });
        initial_rays.push(Ray {
            x: grid[0].len() - 1,
            y,
            direction: Direction::Left,
        })
    }
    for x in 0..grid[0].len() {
        initial_rays.push(Ray {
            x,
            y: 0,
            direction: Direction::Down,
        });
        initial_rays.push(Ray {
            x,
            y: grid.len() - 1,
            direction: Direction::Up,
        })
    }

    initial_rays
        .iter()
        .map(|ray| raycast(&grid, ray.clone()))
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 46);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 51);
    }
}
