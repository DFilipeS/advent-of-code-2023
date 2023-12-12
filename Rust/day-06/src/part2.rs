use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

pub fn process(input: &str) -> String {
    let (_, (time, distance)) = parse_input(input).expect("should parse input");

    (0..=time)
        .filter(|t| distance < (time - t) * t)
        .count()
        .to_string()
}

pub fn parse_input(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, time) = preceded(
        tag("Time:"),
        preceded(space1, separated_list1(space1, digit1)),
    )(input)?;
    let (input, distance) = preceded(
        line_ending,
        preceded(
            tag("Distance: "),
            preceded(space1, separated_list1(space1, digit1)),
        ),
    )(input)?;

    let time: u64 = time.join("").parse().expect("should be a valid number");
    let distance: u64 = distance.join("").parse().expect("should be a valid number");

    Ok((input, (time, distance)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn process_with_example_input() {
        let result = process(INPUT);
        assert_eq!(result, "71503");
    }

    #[test]
    fn parse_input_with_example() {
        let (input, (times, distances)) = parse_input(INPUT).unwrap();
        assert_eq!(input, "");
        assert_eq!(times, 71530);
        assert_eq!(distances, 940200);
    }
}
