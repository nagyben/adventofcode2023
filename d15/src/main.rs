use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    multi::separated_list1,
    sequence::{pair, preceded},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn hash(input: &str) -> usize {
    let mut current_value = 0;

    for c in input.chars() {
        current_value += c as usize;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

fn part1(input: &str) -> usize {
    input.replace('\n', "").split(',').map(hash).sum::<usize>()
}

#[derive(Debug, Clone, Copy)]
struct Lens<'a> {
    label: &'a str,
    focal_length: usize,
}

#[derive(Debug)]
enum Operation<'a> {
    RemoveLens(&'a str),
    InsertLens(&'a str, Lens<'a>),
}

impl Operation<'_> {
    fn parse(input: &str) -> IResult<&str, Operation> {
        let (input, (label, operation)) =
            pair(alpha1, alt((tag("-"), preceded(tag("="), digit1))))(input)?;
        let operation = match operation {
            "-" => Operation::RemoveLens(label),
            focal_length => Operation::InsertLens(
                label,
                Lens {
                    label,
                    focal_length: focal_length.parse().unwrap(),
                },
            ),
        };
        Ok((input, operation))
    }
}

impl PartialEq for Lens<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, operations) = separated_list1(tag(","), Operation::parse)(input)?;
    Ok((input, operations))
}

fn part2(input: &str) -> usize {
    let (_, operations) = parse(input).unwrap();
    let mut hashmap: HashMap<usize, Vec<Lens>> = HashMap::new();
    for operation in &operations {
        match operation {
            Operation::RemoveLens(label) => {
                if let Some(lenses) = hashmap.get_mut(&hash(label)) {
                    if let Some(existing_lens) = lenses.iter().position(|l| &l.label == label) {
                        lenses.remove(existing_lens);
                    }
                }
            }
            Operation::InsertLens(label, lens) => {
                let lenses = hashmap.entry(hash(label)).or_default();
                if let Some(existing_lens) = lenses.iter().position(|l| &l.label == label) {
                    lenses[existing_lens] = *lens;
                } else {
                    lenses.push(*lens);
                }
            }
        }
    }
    hashmap.iter().fold(0, |acc, (boxx, lenses)| {
        acc + lenses.iter().enumerate().fold(0, |acci, (i, lens)| {
            acci + lens.focal_length * (i + 1) * (boxx + 1)
        })
    })
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE1: &str = "HASH";
    static EXAMPLE2: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash() {
        assert_eq!(hash(EXAMPLE1), 52);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE2), 1320)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE2), 145)
    }
}
