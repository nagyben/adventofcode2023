use std::{cmp::max, collections::HashMap};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let cube_limits = HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);
    println!(
        "Part 1: {:?}",
        possible_games(input, &cube_limits).iter().sum::<usize>()
    );
    println!("Part 2: {:?}", cube_power(input));
}

#[derive(Debug, PartialEq)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

impl Game {
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, id) = map_res(preceded(tag("Game "), digit1), |d: &str| d.parse())(input)?;
        let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), Round::parse))(input)?;
        Ok((input, Game { id: id, rounds }))
    }

    fn is_possible(&self, cube_limits: &HashMap<Color, usize>) -> bool {
        self.rounds.iter().all(|r| {
            r.cubes
                .iter()
                .all(|c| c.count <= *cube_limits.get(&c.color).unwrap_or(&(0 as usize)))
        })
    }
}

#[derive(Debug, PartialEq)]
struct Round {
    cubes: Vec<Cube>,
}

#[derive(Debug, PartialEq)]
struct Cube {
    color: Color,
    count: usize,
}

impl Cube {
    // 3 blue
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, count) = map_res(digit1, |d: &str| d.parse())(input)?;
        let (input, color) = preceded(space1, alpha1)(input)?;
        Ok((
            input,
            Cube {
                color: Color::parse(color),
                count: count,
            },
        ))
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn parse(input: &str) -> Self {
        match input {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => panic!("Unknown color {}", input),
        }
    }
}

impl Round {
    // 3 blue, 4 red
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, cubes) = separated_list1(tag(", "), Cube::parse)(input)?;
        Ok((input, Round { cubes }))
    }
}

fn possible_games(input: &str, cube_limits: &HashMap<Color, usize>) -> Vec<usize> {
    let (_, games) = separated_list1(line_ending, Game::parse)(input).unwrap();
    games
        .iter()
        .filter(|g| g.is_possible(cube_limits))
        .map(|g| g.id)
        .collect()
}

fn fewest_cubes(input: &str) -> Vec<HashMap<Color, usize>> {
    let (_, games) = separated_list1(line_ending, Game::parse)(input).unwrap();
    let mut cube_limits =
        vec![HashMap::from([(Color::Red, 0), (Color::Green, 0), (Color::Blue, 0)]); games.len()];
    for (i, game) in games.iter().enumerate() {
        for round in &game.rounds {
            for cube in &round.cubes {
                let color = cube.color.clone();
                let count = cube.count;
                let mut limits = cube_limits[i].clone();
                *limits.entry(color).or_insert(0) = max(count, *limits.get(&color).unwrap_or(&0));
                cube_limits[i] = limits;
            }
        }
    }
    cube_limits
}

fn cube_power(input: &str) -> usize {
    let fewest_cubes = fewest_cubes(input);
    fewest_cubes
        .iter()
        .map(|c| c.values().product::<usize>())
        .sum()
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
        assert_eq!(
            possible_games(
                EXAMPLE_1,
                &HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)])
            ),
            [1, 2, 5]
        )
    }

    #[test]
    fn test_round_parse() {}
    #[test]
    fn test_game_parse() {
        assert_eq!(
            Game::parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
                .unwrap()
                .1,
            Game {
                id: 1,
                rounds: vec![
                    Round {
                        cubes: vec![
                            Cube {
                                color: Color::Blue,
                                count: 3
                            },
                            Cube {
                                color: Color::Red,
                                count: 4
                            }
                        ]
                    },
                    Round {
                        cubes: vec![
                            Cube {
                                color: Color::Red,
                                count: 1
                            },
                            Cube {
                                color: Color::Green,
                                count: 2
                            },
                            Cube {
                                color: Color::Blue,
                                count: 6
                            }
                        ]
                    },
                    Round {
                        cubes: vec![Cube {
                            color: Color::Green,
                            count: 2
                        }]
                    }
                ]
            }
        );
    }

    #[test]
    fn test_fewest_cubes() {
        assert_eq!(
            fewest_cubes(EXAMPLE_1,),
            vec![
                HashMap::from([(Color::Red, 4), (Color::Green, 2), (Color::Blue, 6)]),
                HashMap::from([(Color::Red, 1), (Color::Green, 3), (Color::Blue, 4)]),
                HashMap::from([(Color::Red, 20), (Color::Green, 13), (Color::Blue, 6)]),
                HashMap::from([(Color::Red, 14), (Color::Green, 3), (Color::Blue, 15)]),
                HashMap::from([(Color::Red, 6), (Color::Green, 3), (Color::Blue, 2)]),
            ]
        )
    }

    #[test]
    fn test_cube_power() {
        assert_eq!(cube_power(EXAMPLE_1), 2286)
    }
}
