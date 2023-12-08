use std::{collections::HashMap, thread::current};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
}

type Instructions = Vec<char>;

fn parse(input: &str) -> IResult<&str, (Instructions, HashMap<&str, Vec<&str>>)> {
    let (input, (instructions, network)) = separated_pair(
        alpha1,
        tag("\n\n"),
        separated_list1(line_ending, parse_line),
    )(input)?;
    let network_map = network.iter().cloned().collect();
    Ok((input, (instructions.chars().collect(), network_map)))
}

fn parse_line(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    separated_pair(
        alpha1,
        tag(" = "),
        preceded(
            tag("("),
            terminated(separated_list1(tag(", "), alpha1), tag(")")),
        ),
    )(input)
}

fn run_instructions(network: HashMap<&str, Vec<&str>>, instructions: Instructions) -> usize {
    static TARGET_NODE: &str = "ZZZ";
    let mut cur_node = "AAA";
    let mut cur_instruction_index = 0;
    let mut steps = 0;
    while cur_node != TARGET_NODE {
        dbg!(cur_instruction_index);
        // if we reach the end of the instruction array we wrap around to the start
        if cur_instruction_index >= instructions.len() {
            cur_instruction_index = 0;
        }
        let cur_instruction = instructions[cur_instruction_index];
        let left = network.get(cur_node).unwrap()[0];
        let right = network.get(cur_node).unwrap()[1];
        match cur_instruction {
            'L' => {
                cur_node = left;
            }
            'R' => {
                cur_node = right;
            }
            _ => {
                panic!("Invalid instruction: {}", cur_instruction);
            }
        }
        steps += 1;
        cur_instruction_index += 1;
    }
    steps
}

fn part1(input: &str) -> usize {
    let (_, (instructions, network)) = parse(input).unwrap();
    run_instructions(network, instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

    static EXAMPLE2: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(EXAMPLE1),
            Ok((
                "",
                (
                    vec!['R', 'L'],
                    vec![
                        ("AAA", vec!["BBB", "CCC"]),
                        ("BBB", vec!["DDD", "EEE"]),
                        ("CCC", vec!["ZZZ", "GGG"]),
                        ("DDD", vec!["DDD", "DDD"]),
                        ("EEE", vec!["EEE", "EEE"]),
                        ("GGG", vec!["GGG", "GGG"]),
                        ("ZZZ", vec!["ZZZ", "ZZZ"]),
                    ]
                    .iter()
                    .cloned()
                    .collect()
                )
            ))
        );
        assert_eq!(
            parse(EXAMPLE2),
            Ok((
                "",
                (
                    vec!['L', 'L', 'R'],
                    vec![
                        ("AAA", vec!["BBB", "BBB"]),
                        ("BBB", vec!["AAA", "ZZZ"]),
                        ("ZZZ", vec!["ZZZ", "ZZZ"]),
                    ]
                    .iter()
                    .cloned()
                    .collect()
                )
            ))
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE1), 2);
        assert_eq!(part1(EXAMPLE2), 6);
    }
}
