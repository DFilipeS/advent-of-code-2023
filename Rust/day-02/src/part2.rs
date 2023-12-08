use crate::part1::parse_input;

pub fn process(input: &str) -> String {
    let (_, games) = parse_input(input).expect("should parse input");

    games
        .iter()
        .map(|game| {
            let mut rounds = game.rounds.iter();
            let first = rounds.next().expect("should have at least one round");
            let mut red = first.0;
            let mut green = first.1;
            let mut blue = first.2;

            for round in rounds {
                if round.0 > red {
                    red = round.0;
                }
                if round.1 > green {
                    green = round.1;
                }
                if round.2 > blue {
                    blue = round.2;
                }
            }

            [red, green, blue]
                .iter()
                .filter(|v| **v > 0)
                .product::<u32>()
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn process_with_example_input() {
        let result = process(INPUT);
        assert_eq!(result, "2286");
    }
}
