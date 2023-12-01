#![crate_name = "d01"]

use nom::{branch::alt, bytes::complete::tag, IResult};
fn main() {
    // read the input file
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part_1(input));
}

fn parse_calibration_values(input: &str) -> Vec<i32> {
    let mut calibration_values = Vec::new();
    let mut first_digit: i32;
    let mut second_digit: i32;
    for line in input.lines() {
        let first_char = parse_line(line, ParseDigit::First);
        first_digit = text_to_i32(first_char.unwrap().1);

        let second_char = parse_line(line, ParseDigit::Last);
        second_digit = text_to_i32(second_char.unwrap().1);

        calibration_values.push(first_digit * 10 + second_digit);
    }
    calibration_values
}

enum ParseDigit {
    First,
    Last,
}

fn parse_line(input: &str, parse_digit: ParseDigit) -> IResult<&str, &str> {
    let mut i = input;
    let mut current = "";
    while i.len() > 0 {
        let h: IResult<&str, &str> = alt((
            tag("1"),
            tag("2"),
            tag("3"),
            tag("4"),
            tag("5"),
            tag("6"),
            tag("7"),
            tag("8"),
            tag("9"),
            tag("0"),
            tag("one"),
            tag("two"),
            tag("three"),
            tag("four"),
            tag("five"),
            tag("six"),
            tag("seven"),
            tag("eight"),
            tag("nine"),
        ))(i);
        match h {
            Ok((_, o)) => match parse_digit {
                ParseDigit::First => return Ok((i, o)),
                ParseDigit::Last => {
                    current = o;
                    i = &i[1..];
                }
            },
            Err(_) => i = &i[1..],
        };
    }
    Ok((i, current))
}

fn text_to_i32(input: &str) -> i32 {
    match input {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => input.parse::<i32>().unwrap(),
    }
}

fn part_1(input: &str) -> i32 {
    let calibration_values = parse_calibration_values(input);
    calibration_values.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_1: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

    static INPUT_2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

    #[test]
    fn test_parse() {
        assert_eq!(parse_calibration_values(INPUT_1), vec![12, 38, 15, 77]);
    }

    #[test]
    fn test_parse_2() {
        assert_eq!(
            parse_calibration_values(INPUT_2),
            vec![29, 83, 13, 24, 42, 14, 76]
        );
    }

    #[test]
    fn test_example_1() {
        assert_eq!(part_1(INPUT_1), 142);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(part_1(INPUT_2), 281);
    }
}
