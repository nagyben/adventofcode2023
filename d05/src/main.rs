use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{digit1, line_ending, multispace0, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", lowest_seed_number(input));
}
// each map can be represented an addition operation
// <destination> = input + C
// where C = <destination range start> - <source range start>

// e.g. seed 98 -> soil 50

// source_start = 98
// destination_start = 50
// C = 50 - 98 = -48

// destination = input - 48
//             = 98 - 48
//             = 50

// since the destination start and source start values depend on the input value
// we will need to look up C based on the input value

// let's store the lookups as a struct
#[derive(Debug)]
struct Lookup {
    destination_start: usize,
    source_start: usize,
    length: usize,
}

impl Lookup {
    // returning an Option allows us to use a match statement later on
    // if the input is within the range of the lookup then return the destination
    // else return None
    fn get_destination(&self, input: usize) -> Option<usize> {
        if input >= self.source_start && input <= self.source_start + self.length {
            Some(input + self.destination_start - self.source_start)
        } else {
            None
        }
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, destination_start) =
            preceded(multispace0, map_res(digit1, |d: &str| d.parse::<usize>()))(input)?;
        let (input, source_start) =
            preceded(multispace0, map_res(digit1, |d: &str| d.parse::<usize>()))(input)?;
        let (input, length) =
            preceded(multispace0, map_res(digit1, |d: &str| d.parse::<usize>()))(input)?;
        Ok((
            input,
            Lookup {
                destination_start,
                source_start,
                length,
            },
        ))
    }
}

// first let's parse the input into a vec of Lookups
// the output vector will be a vec of vecs of Lookups
// where
// index 0 = seed-to-soil map
// index 1 = soil-to-fertilizer map
// index 2 = fertilizer-to-water map
// index 3 = water-to-light map
// index 4 = light-to-temperature map
// index 5 = temperature-to-humidity map
// index 6 = humidity-to-location map
fn parse(input: &str) -> IResult<&str, (Vec<usize>, Vec<Vec<Lookup>>)> {
    let (input, seeds) = preceded(
        tag("seeds: "),
        separated_list1(space1, map_res(digit1, |d: &str| d.parse::<usize>())),
    )(input)?;
    let (input, _) = multispace0(input)?;
    let (input, maps) = separated_list1(tag("\n\n"), parse_lookup)(input)?;
    Ok((input, (seeds, maps)))
}

fn parse_lookup(input: &str) -> IResult<&str, Vec<Lookup>> {
    let (input, _) = take_until("\n")(input)?;
    let (input, lookups) = separated_list1(line_ending, Lookup::parse)(input)?;
    Ok((input, lookups))
}

fn seed_to_location(seed: usize, lookups: &Vec<Vec<Lookup>>) -> usize {
    // all we need to do is mutate the seed number based on the lookups

    let mut mapped_value = seed;
    for i in 0..lookups.len() {
        for j in 0..lookups[i].len() {
            // if the seed is within the range of the lookup then update it
            if let Some(destination) = lookups[i][j].get_destination(mapped_value) {
                mapped_value = destination;
                break; // break out of the inner loop since we've found the destination
            }
            // else it stays the same as it was
        }
    }
    mapped_value
}

fn lowest_seed_number(input: &str) -> usize {
    let (_, (seeds, lookups)) = parse(input).unwrap();
    seeds.iter().fold(usize::MAX, |acc, x| {
        let new = seed_to_location(x.clone(), &lookups);
        if new < acc {
            new
        } else {
            acc
        }
    })
}

fn part2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowest_seed_number() {
        let input = include_str!("../example.txt");
        assert_eq!(lowest_seed_number(input), 35);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 46);
    }
}
