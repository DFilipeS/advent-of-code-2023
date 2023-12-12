use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

pub fn process(input: &str) -> String {
    let (_, (times, distances)) = parse_input(input).expect("should parse input");

    times
        .iter()
        .zip(distances)
        .map(|(time, distance)| (0..=*time).filter(|t| distance < (time - t) * t).count())
        .product::<usize>()
        .to_string()
}

pub fn parse_input(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (input, times) = preceded(
        tag("Time:"),
        preceded(space1, separated_list1(space1, complete::u32)),
    )(input)?;

    let (input, distances) = preceded(
        line_ending,
        preceded(
            tag("Distance: "),
            preceded(space1, separated_list1(space1, complete::u32)),
        ),
    )(input)?;

    Ok((input, (times, distances)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn process_with_example_input() {
        let result = process(INPUT);
        assert_eq!(result, "288");
    }

    #[test]
    fn parse_input_with_example() {
        let (input, (times, distances)) = parse_input(INPUT).unwrap();
        assert_eq!(input, "");
        assert_eq!(times, vec![7, 15, 30]);
        assert_eq!(distances, vec![9, 40, 200]);
    }
}
