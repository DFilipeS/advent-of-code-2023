use nom::{
    bytes::complete::tag,
    character::complete::{alpha0, newline},
    multi::separated_list0,
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Round(pub u32, pub u32, pub u32);

#[derive(Debug, PartialEq)]
pub struct Game {
    pub id: u32,
    pub rounds: Vec<Round>,
}

#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Blue,
}

pub fn process(input: &str) -> String {
    let (_, games) = parse_input(input).expect("should parse input");

    games
        .iter()
        .filter_map(|game| {
            for round in game.rounds.iter() {
                if round.0 > 12 || round.1 > 13 || round.2 > 14 {
                    return None;
                }
            }
            Some(game.id)
        })
        .sum::<u32>()
        .to_string()
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list0(newline, parse_game)(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = nom::character::complete::u32(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, rounds) = separated_list0(tag("; "), parse_round)(input)?;

    Ok((input, Game { id, rounds }))
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    let mut round = Round(0, 0, 0);
    let (input, values) = separated_list0(tag(", "), parse_round_value)(input)?;

    for (color, amount) in values {
        match color {
            Colors::Red => round.0 = amount,
            Colors::Green => round.1 = amount,
            Colors::Blue => round.2 = amount,
        }
    }

    Ok((input, round))
}

fn parse_round_value(input: &str) -> IResult<&str, (Colors, u32)> {
    let (input, amount) = nom::character::complete::u32(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = nom::combinator::map(alpha0, |color| match color {
        "red" => Colors::Red,
        "green" => Colors::Green,
        "blue" => Colors::Blue,
        _ => panic!("Unknown color"),
    })(input)?;

    Ok((input, (color, amount)))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", Game{id: 1, rounds: vec![Round(4, 0, 3), Round(1, 2, 6), Round(0, 2, 0)]})]
    #[case("Game 73: 1 green, 1 red, 10 blue; 12 blue; 2 red, 9 blue", Game{id: 73, rounds: vec![Round(1, 1, 10), Round(0, 0, 12), Round(2, 0, 9)]})]
    fn parse_game_with_examples(#[case] input: &str, #[case] expected: Game) {
        let (input, result) = parse_game(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(result, expected);
    }

    #[test]
    fn process_with_example_input() {
        let result = process(INPUT);
        assert_eq!(result, "8");
    }
}
