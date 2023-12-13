use std::cmp::min;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> usize {
    let patterns = parse(input);
    let reflecting_lines: Vec<ReflectingLine> = patterns
        .iter()
        .map(|pattern| find_reflecting_line(pattern).unwrap())
        .collect();
    reflecting_lines.iter().fold(0, |acc, line| match line {
        ReflectingLine::Horizontal(lines) => acc + 100 * lines,
        ReflectingLine::Vertical(lines) => acc + lines,
    })
}

fn part2(input: &str) -> usize {
    let pattern = parse(input);

    


    todo!()
}

type Pattern = Vec<Vec<char>>;

fn parse(input: &str) -> Vec<Pattern> {
    input
        .split("\n\n")
        .map(|x| x.lines().map(|line| line.chars().collect()).collect())
        .collect()
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

#[derive(Debug)]
enum ReflectingLine {
    Horizontal(usize),
    Vertical(usize),
}

fn find_reflecting_line(pattern: &Pattern) -> Result<ReflectingLine, &str> {
    // try horizontals first
    if let Some(value) = reflecting_line(pattern) {
        return Ok(ReflectingLine::Horizontal(value));
    }
    // now transpose and repeat to get the verticals
    let transposed = transpose(pattern.clone());
    if let Some(value) = reflecting_line(&transposed) {
        return Ok(ReflectingLine::Vertical(value));
    }
    Err("No reflecting line found")
}

fn reflecting_line(pattern: &Vec<Vec<char>>) -> Option<usize> {
    for (reflecting_line, _) in pattern[0..pattern.len() - 1].iter().enumerate() {
        let check_range = 0..=min(reflecting_line, pattern.len() - reflecting_line - 2);
        if check_range.into_iter().all(|row_idx| {
            let above = &pattern[reflecting_line - row_idx];
            let below = &pattern[reflecting_line + row_idx + 1];
            above == below
        }) {
            return Some(reflecting_line + 1);
        };
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;
    static EXAMPLE1: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE1), 405);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE1), 400);
    }
}
