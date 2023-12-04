pub fn process(input: &str) -> String {
    parse_input(input).iter().sum::<u32>().to_string()
}

fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> u32 {
    let mut values = line.char_indices().filter_map(|(index, c)| {
        if c.is_ascii_digit() {
            return c.to_digit(10);
        }
        if let Some(value) = parse_spelled_digit(&line[index..]) {
            return Some(value);
        }
        None
    });
    let first = values.next().expect("number should be present");
    let last = values.last().unwrap_or(first);
    format!("{}{}", first, last).parse().unwrap()
}

fn parse_spelled_digit(input: &str) -> Option<u32> {
    let translation_table = vec![
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
    ];
    for (value, identifier) in translation_table {
        if input.starts_with(identifier) {
            return Some(value);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    const INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    #[case("treb7uchet", 77)]
    fn parse_line_with_examples(#[case] input: &str, #[case] expected: u32) {
        let result = parse_line(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn parse_input_with_example() {
        let calibration_values = parse_input(INPUT);
        assert_eq!(calibration_values, vec![29, 83, 13, 24, 42, 14, 76])
    }

    #[test]
    fn process_with_example() {
        let result = process(INPUT);
        assert_eq!(result, "281");
    }
}
