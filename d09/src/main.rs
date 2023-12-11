use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn predict(line: &str) -> i64 {
    let mut diffs: Vec<Vec<i64>> = vec![line
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()];
    while diffs.last().unwrap().iter().collect::<HashSet<_>>().len() != 1 {
        diffs.push(diff(&diffs.last().unwrap()));
    }
    let ends = diffs.iter().map(|d| d.last().unwrap()).collect::<Vec<_>>();
    let mut forecasts = vec![diffs.last().unwrap()[0].clone()];
    for i in 0..ends.len() - 1 {
        let forecast = forecasts.last().unwrap().clone() + ends[i].clone();
        forecasts.push(forecast);
    }
    forecasts.last().unwrap().clone()
}

fn history(line: &str) -> i64 {
    let mut diffs: Vec<Vec<i64>> = vec![line
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()];
    while diffs.last().unwrap().iter().collect::<HashSet<_>>().len() != 1 {
        diffs.push(diff(&diffs.last().unwrap()));
    }
    let mut starts = diffs
        .iter()
        .rev()
        .map(|d| d.first().unwrap())
        .collect::<VecDeque<_>>();
    starts.pop_front();
    let mut histories = vec![diffs.last().unwrap()[0].clone()];
    for i in 0..starts.len() - 1 {
        let history = starts[i].clone() - histories.last().unwrap().clone();
        histories.push(history);
    }
    // do one last time
    histories.push(starts.pop_back().unwrap().clone() - histories.last().unwrap().clone());
    histories.last().unwrap().clone()
}

fn diff(numbers: &Vec<i64>) -> Vec<i64> {
    let mut diffs: Vec<i64> = vec![];
    numbers.windows(2).for_each(|w| {
        diffs.push(w[1] - w[0]);
    });
    diffs
}

fn part1(input: &str) -> isize {
    input
        .lines()
        .map(|line| predict(line))
        .collect::<Vec<_>>()
        .iter()
        .sum::<i64>() as isize
}

fn part2(input: &str) -> isize {
    input
        .lines()
        .map(|line| history(line))
        .collect::<Vec<_>>()
        .iter()
        .sum::<i64>() as isize
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static EXAMPLE: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    #[rstest]
    #[case("0 3 6 9 12 15", 18)]
    #[case("1 3 6 10 15 21", 28)]
    #[case("10 13 16 21 30 45", 68)]
    fn test_predictions(#[case] line: &str, #[case] expected: i64) {
        let actual = predict(line);
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case("0 3 6 9 12 15", -3)]
    #[case("1 3 6 10 15 21", 0)]
    #[case("10 13 16 21 30 45", 5)]
    fn test_history(#[case] line: &str, #[case] expected: i64) {
        let actual = history(line);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1() {
        let actual = part1(EXAMPLE);
        assert_eq!(actual, 114);
    }
    #[test]
    fn test_part2() {
        let actual = part2(EXAMPLE);
        assert_eq!(actual, 2);
    }
}
