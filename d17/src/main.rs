use std::ops::Add;

use image::ImageBuffer;
use nom::{
    bytes::complete::{tag, take, take_until, take_while1},
    character::complete::{digit1, multispace0, one_of},
    combinator::map_res,
    sequence::{preceded, terminated},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

#[derive(Debug, Clone)]
struct Instruction {
    direction: Direction,
    distance: usize,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Direction {
    fn parse(input: char) -> Direction {
        match input {
            'L' => Direction::Left,
            'R' => Direction::Right,
            'D' => Direction::Down,
            'U' => Direction::Up,
            _ => unreachable!("invalid direction"),
        }
    }
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, direction) = one_of("LRDU")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, distance) = map_res(digit1, |s: &str| s.parse::<usize>())(input)?;
        let (input, _) = multispace0(input)?;
        let (input, _) = preceded(tag("("), terminated(parse_color, tag(")")))(input)?;
        Ok((
            input,
            Self {
                direction: Direction::parse(direction),
                distance,
            },
        ))
    }
}

impl Add<&Instruction> for Point {
    type Output = Point;

    fn add(self, instruction: &Instruction) -> Point {
        match instruction.direction {
            Direction::Up => Point {
                x: self.x,
                y: self.y + instruction.distance as isize,
            },
            Direction::Down => Point {
                x: self.x,
                y: self.y - instruction.distance as isize,
            },
            Direction::Left => Point {
                x: self.x - instruction.distance as isize,
                y: self.y,
            },
            Direction::Right => Point {
                x: self.x + instruction.distance as isize,
                y: self.y,
            },
        }
    }
}

fn parse_color(input: &str) -> IResult<&str, String> {
    map_res(
        preceded(tag("#"), take_while1(|c: char| c.is_ascii_hexdigit())),
        |s: &str| s.parse(),
    )(input)
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| Instruction::parse(line).unwrap().1)
        .collect()
}

fn parse2(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| parse_line2(line).unwrap().1)
        .collect()
}

fn parse_line2(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = take_until("#")(input)?;
    let (input, distance) = preceded(
        tag("#"),
        map_res(take(5_usize), |d| usize::from_str_radix(d, 16)),
    )(input)?;
    let (input, direction) = digit1(input)?;
    Ok((
        input,
        Instruction {
            direction: match direction {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "3" => Direction::Up,
                _ => unreachable!(),
            },
            distance,
        },
    ))
}

// Function to calculate the area of a polygon using the shoelace formula
fn calculate_polygon_area(vertices: &Vec<Point>) -> isize {
    let n = vertices.len();

    // Apply the shoelace formula
    let mut area = 0;
    for i in 0..n {
        let j = (i + 1) % n;
        area += vertices[i].x * vertices[j].y;
        area -= vertices[j].x * vertices[i].y;
    }

    area.abs() / 2
}

fn part1(input: &str) -> usize {
    let instructions = parse(input);
    get_area(instructions)
}

fn get_area(instructions: Vec<Instruction>) -> usize {
    let mut points: Vec<Point> = vec![Point { x: 0, y: 0 }];
    for (i, instruction) in instructions.iter().enumerate() {
        points.push(points[i] + instruction)
    }
    calculate_polygon_area(&points) as usize
        + instructions
            .iter()
            .fold(0, |acc, instruction| acc + instruction.distance)
            / 2
        + 1
}

fn part2(input: &str) -> usize {
    let instructions = parse2(input);
    get_area(instructions)
}

fn print_grid(points: &Vec<Point>) {
    // Find the minimum and maximum coordinates to determine the grid size
    let min_x = points.iter().map(|p| p.x).min().unwrap_or(0);
    let max_x = points.iter().map(|p| p.x).max().unwrap_or(0);
    let min_y = points.iter().map(|p| p.y).min().unwrap_or(0);
    let max_y = points.iter().map(|p| p.y).max().unwrap_or(0);

    // Create a 2D grid to represent the points
    let mut grid = vec![vec!['.'; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
    let mut img = ImageBuffer::from_fn(
        (max_x - min_x + 1) as u32,
        (max_y - min_y + 1) as u32,
        |_, _| image::Luma([0u8]),
    );

    // Mark the points on the grid with '#'
    for i in 0..points.len() {
        let point = points[i];
        let x = (point.x - min_x) as usize;
        let y = (max_y - point.y) as usize;
        grid[y][x] = '#';
        img.put_pixel(x as u32, y as u32, image::Luma([255u8]));

        // Connect consecutive points with '#'
        if i > 0 {
            let prev_point = points[i - 1];
            let (start_x, start_y) = (
                (prev_point.x - min_x) as usize,
                (max_y - prev_point.y) as usize,
            );
            let (end_x, end_y) = (x, y);

            // Connect horizontally
            if start_y == end_y {
                for j in std::cmp::min(start_x, end_x)..=std::cmp::max(start_x, end_x) {
                    grid[start_y][j] = '#';
                    img.put_pixel(j as u32, start_y as u32, image::Luma([255u8]));
                }
            }

            // Connect vertically
            if start_x == end_x {
                for j in std::cmp::min(start_y, end_y)..=std::cmp::max(start_y, end_y) {
                    grid[j][start_x] = '#';
                    img.put_pixel(start_x as u32, j as u32, image::Luma([255u8]));
                }
            }
        }
        img.save("test.png").unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 62)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 952408144115)
    }
}
