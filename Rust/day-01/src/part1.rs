pub fn process(input: &str) -> String {
    parse_input(input).iter().sum::<u32>().to_string()
}

fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> u32 {
    let mut values = line.chars().filter(|c| c.is_ascii_digit());
    let first = values.next().expect("number should be present");
    let last = values.last().unwrap_or(first);
    format!("{}{}", first, last).parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn parse_input_with_example() {
        let calibration_values = parse_input(INPUT);
        assert_eq!(calibration_values, vec![12, 38, 15, 77])
    }

    #[test]
    fn process_with_example() {
        let result = process(INPUT);
        assert_eq!(result, "142");
    }
}
