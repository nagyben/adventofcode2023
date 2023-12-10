use std::collections::HashMap;

use nom::{
    character::complete::{self, anychar, line_ending, multispace0},
    multi::{count, separated_list1},
    sequence::separated_pair,
    IResult,
};
fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> usize {
    let mut hands = parse_hands(input).unwrap().1;
    hands.sort();
    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + hand.bid * (i + 1))
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards_map: HashMap<char, usize>,
    cards: Vec<char>,
    bid: usize,
    hand_type: HandType,
}

fn get_hand_type(cards: &Vec<char>, cards_map: &HashMap<char, usize>) -> HandType {
    let mut counts = [0; 5];
    cards.iter().enumerate().for_each(|(idx, c)| {
        counts[idx] = *cards_map.get(c).unwrap();
    });
    counts.sort_unstable();
    match counts {
        [1, 1, 1, 1, 1] => HandType::HighCard,
        [1, 1, 1, 2, 2] => HandType::Pair,
        [1, 2, 2, 2, 2] => HandType::TwoPair,
        [1, 1, 3, 3, 3] => HandType::ThreeOfAKind,
        [2, 2, 3, 3, 3] => HandType::FullHouse,
        [1, 4, 4, 4, 4] => HandType::FourOfAKind,
        [5, 5, 5, 5, 5] => HandType::FiveOfAKind,
        c => todo!("Hand type not implemented: {:?}", c),
    }
}

impl Hand {
    fn parse(input: &str) -> IResult<&str, Hand> {
        let (input, (cards, bid)) =
            separated_pair(count(anychar, 5), multispace0, complete::u32)(input)?;
        let mut cards_map = HashMap::new();
        for c in &cards {
            let d = cards_map.entry(*c).or_insert(0);
            *d += 1;
        }
        let hand_type = get_hand_type(&cards, &cards_map);
        Ok((
            input,
            Hand {
                cards,
                cards_map,
                bid: bid as usize,
                hand_type,
            },
        ))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let c = self.hand_type.cmp(&other.hand_type);
        match c {
            std::cmp::Ordering::Equal => {
                for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                    let c = compare_cards(self_card, other_card);
                    if c != std::cmp::Ordering::Equal {
                        return c;
                    }
                }
                std::cmp::Ordering::Equal
            }
            _ => c,
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn compare_cards(a: &char, b: &char) -> std::cmp::Ordering {
    let a = match a {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        c => c.to_digit(10).unwrap(),
    };
    let b = match b {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        c => c.to_digit(10).unwrap(),
    };
    a.cmp(&b)
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            std::cmp::Ordering::Equal
        } else {
            static ORDER: [HandType; 7] = [
                HandType::HighCard,
                HandType::Pair,
                HandType::TwoPair,
                HandType::ThreeOfAKind,
                HandType::FullHouse,
                HandType::FourOfAKind,
                HandType::FiveOfAKind,
            ];
            // compare the indexes of the hand type in the ORDER array
            ORDER
                .iter()
                .position(|&x| x == *self)
                .unwrap()
                .cmp(&ORDER.iter().position(|&x| x == *other).unwrap())
        }
    }
}
impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_hands(input: &str) -> IResult<&str, Vec<Hand>> {
    let (input, hands) = separated_list1(line_ending, Hand::parse)(input).unwrap();
    Ok((input, hands))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    const EXAMPLE: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    const EXAMPLE2: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
QQQQA 483
QQQQQ 483"#;

    #[test]
    fn test_example_1() {
        assert_eq!(part1(EXAMPLE), 6440);
    }

    #[test]
    fn test_parse_hands() {
        assert_eq!(
            parse_hands(EXAMPLE2),
            Ok((
                "",
                vec![
                    Hand {
                        cards: vec!['3', '2', 'T', '3', 'K'],
                        cards_map: vec![('3', 2), ('2', 1), ('T', 1), ('K', 1)]
                            .iter()
                            .cloned()
                            .collect(),
                        bid: 765,
                        hand_type: HandType::Pair,
                    },
                    Hand {
                        cards: vec!['T', '5', '5', 'J', '5'],
                        cards_map: vec![('T', 1), ('5', 3), ('J', 1)].iter().cloned().collect(),
                        bid: 684,
                        hand_type: HandType::ThreeOfAKind,
                    },
                    Hand {
                        cards: vec!['K', 'K', '6', '7', '7'],
                        cards_map: vec![('K', 2), ('6', 1), ('7', 2)].iter().cloned().collect(),
                        bid: 28,
                        hand_type: HandType::TwoPair,
                    },
                    Hand {
                        cards: vec!['K', 'T', 'J', 'J', 'T'],
                        cards_map: vec![('K', 1), ('T', 2), ('J', 2)].iter().cloned().collect(),
                        bid: 220,
                        hand_type: HandType::TwoPair,
                    },
                    Hand {
                        cards: vec!['Q', 'Q', 'Q', 'J', 'A'],
                        cards_map: vec![('Q', 3), ('J', 1), ('A', 1)].iter().cloned().collect(),
                        bid: 483,
                        hand_type: HandType::ThreeOfAKind,
                    },
                    Hand {
                        cards: vec!['Q', 'Q', 'Q', 'Q', 'A'],
                        cards_map: vec![('Q', 4), ('A', 1)].iter().cloned().collect(),
                        bid: 483,
                        hand_type: HandType::FourOfAKind,
                    },
                    Hand {
                        cards: vec!['Q', 'Q', 'Q', 'Q', 'Q'],
                        cards_map: vec![('Q', 5)].iter().cloned().collect(),
                        bid: 483,
                        hand_type: HandType::FiveOfAKind,
                    }
                ]
            ))
        );
    }

    #[test]
    fn test_sort_hands() {
        let mut hands = parse_hands(EXAMPLE2).unwrap().1;
        hands.sort();
        let expected = vec![
            Hand::parse("32T3K 765").unwrap().1,
            Hand::parse("KTJJT 220").unwrap().1,
            Hand::parse("KK677 28").unwrap().1,
            Hand::parse("T55J5 684").unwrap().1,
            Hand::parse("QQQJA 483").unwrap().1,
            Hand::parse("QQQQA 483").unwrap().1,
            Hand::parse("QQQQQ 483").unwrap().1,
        ];
        dbg!(&hands.iter().map(|h| &h.cards).collect::<Vec<_>>());
        dbg!(&expected.iter().map(|h| &h.cards).collect::<Vec<_>>());
        assert_eq!(hands, expected);
    }
}
