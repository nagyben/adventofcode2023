use std::cmp::min;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let patterns = parse(input);
    let reflecting_lines: Vec<ReflectingLine> = patterns
        .iter()
        .map(|pattern| find_reflecting_line(pattern).unwrap())
        .collect();
    // dbg!(&reflecting_lines.iter().count());
    // dbg!(&reflecting_lines);
    reflecting_lines.iter().fold(0, |acc, line| match line {
        ReflectingLine::Horizontal(lines) => acc + 100 * lines,
        ReflectingLine::Vertical(lines) => acc + lines,
    })
}

#[derive(Debug)]
struct PatternPermutation {
    pattern: Pattern,
    permutation_index: (usize, usize),
}

impl Iterator for PatternPermutation {
    type Item = Pattern;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.permutation_index;
        let mut new_pattern = self.pattern.clone();
        // dbg!(self.permutation_index);

        if y >= self.pattern.len() {
            return None;
        }

        new_pattern[y][x] = match new_pattern[y][x] {
            '.' => '#',
            '#' => '.',
            _ => panic!("Invalid character"),
        };

        if x < self.pattern[0].len() - 1 {
            self.permutation_index = (x + 1, y);
        } else {
            self.permutation_index = (0, y + 1);
        }

        // dbg!(self
        //     .pattern
        //     .iter()
        //     .map(|d| d.iter().collect::<String>())
        //     .collect::<Vec<_>>());
        // dbg!(new_pattern
        //     .iter()
        //     .map(|d| d.iter().collect::<String>())
        //     .collect::<Vec<_>>());
        Some(new_pattern)
    }
}

fn part2(input: &str) -> usize {
    let original_patterns = parse(input);
    let patterns: Vec<PatternPermutation> = parse(input)
        .iter()
        .map(|pattern| PatternPermutation {
            pattern: pattern.clone(),
            permutation_index: (0, 0),
        })
        .collect();

    let original_reflecting_lines: Vec<ReflectingLine> = original_patterns
        .iter()
        .map(|pattern| find_reflecting_line(pattern).unwrap())
        .collect();

    let mut reflecting_lines: Vec<ReflectingLine> = vec![];
    dbg!(original_reflecting_lines.len());
    dbg!(patterns.len());
    for (pattern, original_reflecting_line) in patterns
        .into_iter()
        .zip(original_reflecting_lines.into_iter())
    {
        // dbg!(&pattern
        //     .pattern
        //     .iter()
        //     .map(|d| d.iter().collect::<String>())
        //     .collect::<Vec<_>>());
        let mut n_permutations = 0;
        for permutation in pattern.into_iter() {
            n_permutations += 1;
            match find_reflecting_line(&permutation) {
                Ok(line) => {
                    if line == original_reflecting_line {
                        continue;
                    }
                    println!("Found a new reflecting line {:?}", &line);
                    reflecting_lines.push(line);
                    break;
                }
                Err(_) => continue,
            }
            dbg!(n_permutations);
        }
    }
    // dbg!(n_permutations);
    // dbg!(&reflecting_lines);
    dbg!(reflecting_lines.len());
    reflecting_lines.iter().fold(0, |acc, line| match line {
        ReflectingLine::Horizontal(lines) => acc + 100 * lines,
        ReflectingLine::Vertical(lines) => acc + lines,
    })
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

#[derive(Debug, PartialEq)]
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
        let check_range = 0..=reflecting_line;
        if check_range.into_iter().all(|row_idx| {
            if (reflecting_line + row_idx + 1) <= pattern.len() - 1 {
                let above = &pattern[reflecting_line - row_idx];
                let below = &pattern[reflecting_line + row_idx + 1];
                above == below
            } else {
                true
            }
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
        // assert_eq!(part1(EXAMPLE1), 40);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE1), 400);
        assert_eq!(part2(EXAMPLE1), 0);
    }
}
