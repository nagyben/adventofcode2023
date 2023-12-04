use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", scratchcard_points(input));
}

fn scratchcard_points(input: &str) -> u32 {
    let (_, cards) = separated_list1(tag("\n"), parse_scratchcard)(input).unwrap();
    cards.iter().fold(0, |total, (card_numbers, my_numbers)| {
        let out = total
            + my_numbers.iter().fold(0, |acc, my_number| {
                if card_numbers.contains(my_number) {
                    if acc == 0 {
                        1
                    } else {
                        acc * 2
                    }
                } else {
                    acc
                }
            });
        out
    })
}

fn parse_scratchcards(input: &str) -> IResult<&str, Vec<(Vec<u32>, Vec<u32>)>> {
    separated_list1(tag("\n"), parse_scratchcard)(input)
}

fn parse_scratchcard(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = digit1(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, (card_numbers, my_numbers)) = separated_pair(
        separated_list1(
            space1,
            map_res(preceded(space0, digit1), |s: &str| s.parse::<u32>()),
        ),
        tag(" | "),
        separated_list1(
            space1,
            map_res(preceded(space0, digit1), |s: &str| s.parse::<u32>()),
        ),
    )(input)?;
    Ok((input, (card_numbers, my_numbers)))
}

#[cfg(test)]
mod test {
    use super::*;
    const EXAMPLE1: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn test_scratchcard_points() {
        assert_eq!(scratchcard_points(EXAMPLE1), 13);
    }

    #[test]
    fn test_parse_scratchcard() {
        assert_eq!(
            parse_scratchcards(EXAMPLE1),
            Ok((
                "",
                vec![
                    (vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53]),
                    (
                        vec![13, 32, 20, 16, 61],
                        vec![61, 30, 68, 82, 17, 32, 24, 19]
                    ),
                    (vec![1, 21, 53, 59, 44], vec![69, 82, 63, 72, 16, 21, 14, 1]),
                    (
                        vec![41, 92, 73, 84, 69],
                        vec![59, 84, 76, 51, 58, 5, 54, 83]
                    ),
                    (
                        vec![87, 83, 26, 28, 32],
                        vec![88, 30, 70, 12, 93, 22, 82, 36]
                    ),
                    (
                        vec![31, 18, 13, 56, 72],
                        vec![74, 77, 10, 23, 35, 67, 36, 11]
                    )
                ]
            ))
        );
    }
}
