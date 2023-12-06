use nom::{
    bytes::complete::tag,
    character::complete,
    character::complete::space1,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let races = parse(input).unwrap().1;
    let mut wins: Vec<u64> = vec![0; races.len()];

    races.iter().enumerate().for_each(|(race_number, race)| {
        wins[race_number] = ways_to_win(race);
    });
    wins.iter().product::<u64>() as usize
}

fn part2(input_: &str) -> usize {
    let input = input_.replace(' ', "");
    let input = input.strip_suffix('\n').unwrap_or(input.as_str());
    let (_, (time, distance)) = parse_part2(input).unwrap();

    let race = Race {
        time,
        record: distance,
    };
    ways_to_win(&race) as usize
}

fn ways_to_win(race: &Race) -> u64 {
    // quadratic formula
    // x = -b +- sqrt(b^2 - 4ac) / 2a

    let a = 1_f64;
    let b = -(race.time as f64);
    let c = race.record as f64;
    // min = -b - sqrt(b^2 - 4ac) / 2a
    // max = -b + sqrt(b^2 - 4ac) / 2a
    let min_waiting_time = (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a) + 0.001; // add a tiny bit because if the root is a whole number then we need to round to the next whole number
    let max_waiting_time = (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a) - 0.001; // same story

    (max_waiting_time.floor() - min_waiting_time.ceil()) as u64 + 1
}

#[derive(Debug, PartialEq)]
struct Race {
    time: u64,
    record: u64,
}

type Races = Vec<Race>;

fn parse(input: &str) -> IResult<&str, Races> {
    let (input, (times, distances)) = separated_pair(
        preceded(
            preceded(tag("Time:"), space1),
            separated_list1(space1, complete::u64),
        ),
        tag("\n"),
        preceded(
            preceded(tag("Distance:"), space1),
            separated_list1(space1, complete::u64),
        ),
    )(input)?;
    let races = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| Race {
            time: *time,
            record: *distance,
        })
        .collect();
    Ok((input, races))
}

fn parse_part2(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, (time, distance)) = separated_pair(
        preceded(tag("Time:"), complete::u64),
        tag("\n"),
        preceded(tag("Distance:"), complete::u64),
    )(input)?;
    Ok((input, (time, distance)))
}

#[cfg(test)]
mod tests {

    use super::*;

    const EXAMPLE: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn test_example1() {
        assert_eq!(part1(EXAMPLE), 288)
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(EXAMPLE).unwrap().1,
            vec![
                Race { time: 7, record: 9 },
                Race {
                    time: 15,
                    record: 40
                },
                Race {
                    time: 30,
                    record: 200
                }
            ]
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 71503)
    }
}
