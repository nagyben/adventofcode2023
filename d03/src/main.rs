fn main() {
    let input = include_str!("../input.txt");
    println!("Sum of part numbers: {:?}", sum_of_part_numbers(input));
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn part_numbers(input: &str) -> Vec<u32> {
    let schematic = parse(input);
    let mut current_digits: Vec<char> = vec![];
    let mut part_numbers: Vec<u32> = vec![];
    let mut is_part_number = false;
    schematic.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, c)| {
            if c.is_ascii_digit() {
                current_digits.push(c.clone());
                // check if the digit is a part number
                // check a 3x3 grid centered on the digit
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
                        if !schematic[y][x].is_numeric() && schematic[y][x] != '.' {
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
    part_numbers(input).iter().sum::<u32>()
}

fn gears(input: &str) -> Vec<(u16, u16)> {
    let schematic = parse(input);
    let mut gears: Vec<(u16, u16)> = vec![];

    schematic.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, c)| {
            if *c == '*' {
                // the numbers are maximum 3 digits
                // assume that each gear will only be surrounded by 2 numbers
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
                        if schematic[y][x].is_numeric() {
                            gear.0 = schematic[y][x].to_digit(10).unwrap() as u16;
                        }
                        if schematic[y][x] == '+' {
                            gear.1 += 1;
                        }
                    })
                });
                gears.push(gear);
            }
        });
    });
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
            part_numbers(EXAMPLE),
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
}
