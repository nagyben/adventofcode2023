use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, space0, space1},
    combinator::{map_res, opt},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
struct Game {
    id: usize,
    cube_sets: Vec<CubeSet>,
}

impl Game {
    // we can use this method with other nom combinator parsers
    fn parse(input: &str) -> IResult<&str, Self> {
        dbg!(input);
        let (input, _) = tag("Game ")(input)?; // trim "Game: " from the beginning
        let (input, id) = map_res(digit1, |d: &str| d.parse())(input)?; // parse the game number
        let (input, _) = space0(input)?; // trim any whitespace after the game number
        let (input, _) = tag(": ")(input)?; // trim "Game: " from the beginning
        
        let (input, cube_sets) = separated_list1(tag(";"), alpha1)(input)?;
        let (input, cube_sets) = separated_list1(tag(";"), CubeSet::parse)(input)?;
        Ok((
            input,
            Game {
                id: id,
                cube_sets: cube_sets,
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl CubeSet {
    // we can use this method with other nom combinator parsers
    fn parse(input: &str) -> IResult<&str, Self> {
        // input string gonna look like "1 red, 2 green, 6 blue"
        // although the order of the colors is not guaranteed
        // and not all colors are guaranteed to be present
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        let mut input = input;

        dbg!(input);
        while !input.is_empty() {
            let i = input;
            let (i, _) = space0(i)?; // trim leading whitespace if any
            let (i, _) = opt(tag(","))(i)?; // trim commas if any
            let (i, _) = space0(i)?; // trim remaining whitespace if any
            let (i, (count, color)) = separated_pair(digit1, space1, alpha1)(i)?; // parse the count
            input = i;

            match color {
                "red" => red = count.parse().unwrap(),
                "green" => green = count.parse().unwrap(),
                "blue" => blue = count.parse().unwrap(),
                _ => panic!("unexpected color"),
            }
        }

        Ok((input, CubeSet { red, green, blue }))
    }
}

fn possible_games(input: &str) -> Vec<usize> {
    let (_, games) = separated_list1(tag("\n"), Game::parse)(input).unwrap();
    let mut possible_games = Vec::new();

    possible_games
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_1: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn test_example_1() {
        assert_eq!(possible_games(EXAMPLE_1), [1, 2, 5]);
    }

    #[test]
    fn test_cubeset_parse() {
        assert_eq!(
            CubeSet::parse("1 red, 2 green, 6 blue").unwrap().1,
            CubeSet {
                red: 1,
                green: 2,
                blue: 6
            }
        );
        assert_eq!(
            CubeSet::parse("1 red, 2 green").unwrap().1,
            CubeSet {
                red: 1,
                green: 2,
                blue: 0
            }
        );
        assert_eq!(
            CubeSet::parse("1 red, 6 blue").unwrap().1,
            CubeSet {
                red: 1,
                green: 0,
                blue: 6
            }
        );
        assert_eq!(
            CubeSet::parse("2 green, 6 blue").unwrap().1,
            CubeSet {
                red: 0,
                green: 2,
                blue: 6
            }
        );
        assert_eq!(
            CubeSet::parse("1 red").unwrap().1,
            CubeSet {
                red: 1,
                green: 0,
                blue: 0
            }
        );
        assert_eq!(
            CubeSet::parse("2 green").unwrap().1,
            CubeSet {
                red: 0,
                green: 2,
                blue: 0
            }
        );
        assert_eq!(
            CubeSet::parse("6 blue").unwrap().1,
            CubeSet {
                red: 0,
                green: 0,
                blue: 6
            }
        );
    }
    #[test]
    fn test_game_parse() {
        assert_eq!(
            Game::parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
                .unwrap()
                .1,
            Game {
                id: 1,
                cube_sets: vec![
                    CubeSet {
                        red: 4,
                        green: 0,
                        blue: 3
                    },
                    CubeSet {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    CubeSet {
                        red: 0,
                        green: 2,
                        blue: 0
                    }
                ]
            }
        );
    }
}
