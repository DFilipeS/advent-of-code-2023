use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

use nom::character::complete::{self, alphanumeric1};
use nom::{
    character::complete::{line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Card {
    Number(u32),
    Jack,
    Queen,
    King,
    Ace,
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Self::Number(2)),
            "3" => Ok(Self::Number(3)),
            "4" => Ok(Self::Number(4)),
            "5" => Ok(Self::Number(5)),
            "6" => Ok(Self::Number(6)),
            "7" => Ok(Self::Number(7)),
            "8" => Ok(Self::Number(8)),
            "9" => Ok(Self::Number(9)),
            "T" => Ok(Self::Number(10)),
            "J" => Ok(Self::Jack),
            "Q" => Ok(Self::Queen),
            "K" => Ok(Self::King),
            "A" => Ok(Self::Ace),
            _ => Err(String::from("Invalid card")),
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Card::Number(val_1), Card::Number(val_2)) => Some(val_1.cmp(val_2)),
            (Card::Number(_), _) => Some(Ordering::Less),
            (_, Card::Number(_)) => Some(Ordering::Greater),
            (Card::Jack, Card::Jack) => Some(Ordering::Equal),
            (Card::Jack, Card::Queen) => Some(Ordering::Less),
            (Card::Jack, Card::King) => Some(Ordering::Less),
            (Card::Jack, Card::Ace) => Some(Ordering::Less),
            (Card::Queen, Card::Jack) => Some(Ordering::Greater),
            (Card::Queen, Card::Queen) => Some(Ordering::Equal),
            (Card::Queen, Card::King) => Some(Ordering::Less),
            (Card::Queen, Card::Ace) => Some(Ordering::Less),
            (Card::King, Card::Jack) => Some(Ordering::Greater),
            (Card::King, Card::Queen) => Some(Ordering::Greater),
            (Card::King, Card::King) => Some(Ordering::Equal),
            (Card::King, Card::Ace) => Some(Ordering::Less),
            (Card::Ace, Card::Jack) => Some(Ordering::Greater),
            (Card::Ace, Card::Queen) => Some(Ordering::Greater),
            (Card::Ace, Card::King) => Some(Ordering::Greater),
            (Card::Ace, Card::Ace) => Some(Ordering::Equal),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq)]
pub struct Hand(Vec<Card>);

impl Hand {
    fn get_type(&self) -> HandType {
        let mut frequencies: Vec<_> = self
            .0
            .iter()
            .fold(HashMap::new(), |mut map, card| {
                map.entry(card).and_modify(|f| *f += 1).or_insert(1);
                map
            })
            .into_values()
            .collect();
        frequencies.sort();

        match frequencies.as_slice() {
            [5] => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let result = self.get_type().partial_cmp(&other.get_type());
        if let Some(Ordering::Equal) = result {
            for (c1, c2) in self.0.iter().zip(other.0.iter()) {
                match c1.partial_cmp(c2) {
                    Some(Ordering::Equal) => (),
                    Some(ordering) => return Some(ordering),
                    _ => (),
                }
            }
        }
        result
    }
}

pub type Bid = u32;

pub fn process(input: &str) -> String {
    let (_, mut hands) = parse_input(input).expect("should parse input");
    hands.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

    hands
        .iter()
        .zip(1..)
        .map(|((_, bid), i)| *bid * i)
        .sum::<u32>()
        .to_string()
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<(Hand, Bid)>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, (Hand, Bid)> {
    separated_pair(parse_hand, space1, complete::u32)(input)
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, hand) = alphanumeric1(input)?;
    let hand = hand
        .split("")
        .filter_map(|card| card.parse().ok())
        .collect::<Vec<Card>>();
    Ok((input, Hand(hand)))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn process_with_example_input() {
        let result = process(INPUT);
        assert_eq!(result, "6440");
    }

    #[rstest]
    #[case(Hand(vec![Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace]), Hand(vec![Card::Ace, Card::Ace, Card::Number(8), Card::Ace, Card::Ace]), Some(Ordering::Greater))]
    #[case(Hand(vec![Card::Ace, Card::Ace, Card::Number(8), Card::Ace, Card::Ace]), Hand(vec![Card::Ace, Card::Ace, Card::Number(8), Card::Ace, Card::Ace]), Some(Ordering::Equal))]
    #[case(Hand(vec![Card::Ace, Card::Ace, Card::Number(8), Card::Ace, Card::Ace]), Hand(vec![Card::Ace, Card::Ace, Card::Number(9), Card::Ace, Card::Ace]), Some(Ordering::Less))]
    fn hand_type_partial_ord_with_example_input(
        #[case] a: Hand,
        #[case] b: Hand,
        #[case] expected: Option<Ordering>,
    ) {
        let result = a.partial_cmp(&b);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(Hand(vec![Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace]), HandType::FiveOfAKind)]
    #[case(Hand(vec![Card::Ace, Card::Ace, Card::Number(8), Card::Ace, Card::Ace]), HandType::FourOfAKind)]
    #[case(Hand(vec![Card::Number(2), Card::Number(3), Card::Number(3), Card::Number(3), Card::Number(2)]), HandType::FullHouse)]
    #[case(Hand(vec![Card::Number(10), Card::Number(10), Card::Number(10), Card::Number(9), Card::Number(8)]), HandType::ThreeOfAKind)]
    #[case(Hand(vec![Card::Number(2), Card::Number(3), Card::Number(4), Card::Number(3), Card::Number(2)]), HandType::TwoPair)]
    #[case(Hand(vec![Card::Ace, Card::Number(2), Card::Number(3), Card::Ace, Card::Number(4)]), HandType::OnePair)]
    #[case(Hand(vec![Card::Number(2), Card::Number(3), Card::Number(4), Card::Number(5), Card::Number(6)]), HandType::HighCard)]
    fn hand_get_type(#[case] hand: Hand, #[case] expected: HandType) {
        let result = hand.get_type();
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_input_with_example() {
        let (input, hands) = parse_input(INPUT).unwrap();
        assert_eq!(input, "");
        assert_eq!(
            hands,
            vec![
                (
                    Hand(vec![
                        Card::Number(3),
                        Card::Number(2),
                        Card::Number(10),
                        Card::Number(3),
                        Card::King
                    ]),
                    765
                ),
                (
                    Hand(vec![
                        Card::Number(10),
                        Card::Number(5),
                        Card::Number(5),
                        Card::Jack,
                        Card::Number(5)
                    ]),
                    684
                ),
                (
                    Hand(vec![
                        Card::King,
                        Card::King,
                        Card::Number(6),
                        Card::Number(7),
                        Card::Number(7)
                    ]),
                    28
                ),
                (
                    Hand(vec![
                        Card::King,
                        Card::Number(10),
                        Card::Jack,
                        Card::Jack,
                        Card::Number(10)
                    ]),
                    220,
                ),
                (
                    Hand(vec![
                        Card::Queen,
                        Card::Queen,
                        Card::Queen,
                        Card::Jack,
                        Card::Ace
                    ]),
                    483,
                ),
            ]
        );
    }
}
