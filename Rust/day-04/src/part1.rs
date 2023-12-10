use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space0, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Card {
    pub id: u32,
    pub winners: Vec<u32>,
    pub numbers: Vec<u32>,
}

impl Card {
    pub fn count_winners(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|number| self.winners.contains(number))
            .count()
            .try_into()
            .unwrap()
    }
}

pub fn process(input: &str) -> String {
    let (_, cards) = parse_input(input).expect("should parse input");

    cards
        .iter()
        .filter_map(|card| {
            let count_winner_numbers = card.count_winners();
            if count_winner_numbers > 0 {
                return Some(2_u32.pow(count_winner_numbers - 1));
            }
            None
        })
        .sum::<u32>()
        .to_string()
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, parse_card)(input)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, id) = preceded(tag("Card "), preceded(space0, complete::u32))(input)?;
    let (input, (winners, numbers)) = preceded(
        tag(": "),
        separated_pair(parse_numbers, tag(" | "), parse_numbers),
    )(input)?;
    let card = Card {
        id,
        winners,
        numbers,
    };

    Ok((input, card))
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(space0, separated_list1(space1, complete::u32))(input)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", Card{id: 1, winners: vec![41, 48, 83, 86, 17], numbers: vec![83, 86, 6, 31, 17, 9, 48, 53]})]
    #[case("Card  1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", Card{id: 1, winners: vec![41, 48, 83, 86, 17], numbers: vec![83, 86, 6, 31, 17, 9, 48, 53]})]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", Card{id: 3, winners: vec![1, 21, 53, 59, 44], numbers: vec![69, 82, 63, 72, 16, 21, 14, 1]})]
    #[case("Card 23: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", Card{id: 23, winners: vec![41, 48, 83, 86, 17], numbers: vec![83, 86, 6, 31, 17, 9, 48, 53]})]
    fn parse_card_with_examples(#[case] input: &str, #[case] expected: Card) {
        let (input, result) = parse_card(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(result, expected);
    }

    #[test]
    fn process_with_example_input() {
        let result = process(INPUT);
        assert_eq!(result, "13");
    }
}
