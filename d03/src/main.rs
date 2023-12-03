use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    println!("Sum of part numbers: {:?}", sum_of_part_numbers(input));
    println!("Sum of gear ratios: {:?}", sum_of_gears(input));
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn part_numbers(input: &str, symbols: Option<&HashSet<char>>) -> Vec<u32> {
    let schematic = parse(input);
    part_numbers_from_schematic(&schematic, symbols)
}

fn part_numbers_from_schematic(
    schematic: &Vec<Vec<char>>,
    symbols: Option<&HashSet<char>>,
) -> Vec<u32> {
    let mut current_digits: Vec<char> = vec![]; // scratch space for the current number sequence
    let mut part_numbers: Vec<u32> = vec![]; // the output vector
    let mut is_part_number = false; // marker to indicate if the current number is a part number
    schematic.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, c)| {
            if c.is_ascii_digit() {
                // store the current number in the scratch space
                current_digits.push(c.clone());
                // check a 3x3 grid centered on the digit and see if there are any symbols around
                let range_y = if y == 0 { y } else { y - 1 }..=if y == schematic.len() - 1 {
                    schematic.len() - 1
                } else {
                    y + 1
                };
                range_y.into_iter().for_each(|y| {
                    let range_x = if x == 0 { x } else { x - 1 }..=if x == row.len() - 1 {
                        row.len() - 1
                    } else {
                        x + 1
                    };
                    range_x.into_iter().for_each(|x| {
                        if let Some(symbols) = symbols {
                            // TODO: this is a dumb way to do this but it works
                            if y == 1 && x == 3 && symbols.contains(&schematic[y][x]) {
                                is_part_number = true;
                            }
                        } else if !schematic[y][x].is_numeric() && schematic[y][x] != '.' {
                            is_part_number = true;
                        }
                    })
                });
            }

            // we need to handle the edge case where the number ends at the end of a row
            if !c.is_ascii_digit() || x == row.len() - 1 {
                if !current_digits.is_empty() && is_part_number {
                    let part_number = current_digits
                        .iter()
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap();
                    part_numbers.push(part_number);
                }
                current_digits = vec![];
                is_part_number = false;
            }
        });
    });
    part_numbers
}

fn sum_of_part_numbers(input: &str) -> u32 {
    part_numbers(input, None).iter().sum::<u32>()
}

fn gears(input: &str) -> Vec<(u32, u32)> {
    let schematic = parse(input);
    let mut gears: Vec<(u32, u32)> = vec![];

    // we can search for the gear symbols and then create a 3x7 (Y*X) subgrid around it
    // we can use our previous function part_numbers on each of these subgrids to find the part numbers
    // if the previous function returns two numbers then we can add them to the gears vector
    schematic.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, c)| {
            if *c == '*' {
                let range_y = if y == 0 { y } else { y - 1 }..=if y == schematic.len() - 1 {
                    schematic.len() - 1
                } else {
                    y + 1
                };
                let range_x = if x == 0 { x } else { x - 3 }..=if x == row.len() - 1 {
                    row.len() - 1
                } else {
                    x + 3
                };
                let subgrid = range_y
                    .into_iter()
                    .map(|y| {
                        range_x
                            .clone()
                            .into_iter()
                            .map(|x| schematic[y][x])
                            .collect::<Vec<char>>()
                    })
                    .collect::<Vec<Vec<char>>>();

                // now run part_numbers on the subgrid
                // with only the '*' symbol allowed
                let part_numbers =
                    part_numbers_from_schematic(&subgrid, Some(&HashSet::from(['*'])));
                if part_numbers.len() == 2 {
                    gears.push((part_numbers[0] as u32, part_numbers[1] as u32));
                }
            }
        });
    });
    gears
}

fn sum_of_gears(input: &str) -> u32 {
    gears(input).iter().map(|(a, b)| a * b).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn test_part_numbers() {
        assert_eq!(
            part_numbers(EXAMPLE, None),
            vec![467, 35, 633, 617, 592, 755, 664, 598]
        );
    }

    #[test]
    fn test_sum_of_part_numbers() {
        assert_eq!(sum_of_part_numbers(EXAMPLE), 4361);
    }

    #[test]
    fn test_gears() {
        assert_eq!(gears(EXAMPLE), vec![(467, 35), (755, 598)]);
    }

    #[test]
    fn test_sum_of_gears() {
        assert_eq!(sum_of_gears(EXAMPLE), 467835);
    }
}
