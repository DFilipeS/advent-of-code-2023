use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, anychar, line_ending},
    combinator::map,
    multi::{many_till, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug)]
pub enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("unknown instruction"),
        }
    }
}

pub type Nodes<'a> = BTreeMap<&'a str, (&'a str, &'a str)>;

pub fn process(input: &str) -> String {
    let (_, (instructions, nodes)) = parse_input(input).unwrap();
    let mut index = 0;
    let mut cursor = "AAA";

    loop {
        if cursor == "ZZZ" {
            break;
        }
        let node = nodes.get(cursor).expect("node should exist");
        let instruction = instructions
            .get(index % instructions.len())
            .expect("instruction should exist");

        cursor = match instruction {
            Instruction::Left => node.0,
            Instruction::Right => node.1,
        };

        index += 1;
    }

    index.to_string()
}

pub fn parse_input(input: &str) -> IResult<&str, (Vec<Instruction>, Nodes)> {
    let (input, instructions) = parse_instructions(input)?;
    let (input, nodes) = parse_nodes(input)?;

    Ok((input, (instructions, nodes)))
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, (instructions, _)) =
        many_till(map(anychar, Instruction::from), line_ending)(input)?;
    let (input, _) = line_ending(input)?;

    Ok((input, instructions))
}

fn parse_nodes(input: &str) -> IResult<&str, Nodes> {
    let mut nodes = BTreeMap::new();
    let (input, values) = separated_list1(line_ending, parse_node)(input)?;

    for (key, value) in values {
        nodes.insert(key, value);
    }

    Ok((input, nodes))
}

fn parse_node(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    let parse_value = delimited(
        tag("("),
        separated_pair(alphanumeric1, tag(", "), alphanumeric1),
        tag(")"),
    );
    let (input, (key, value)) = separated_pair(alphanumeric1, tag(" = "), parse_value)(input)?;

    Ok((input, (key, value)))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    const INPUT_EXAMPLE_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const INPUT_EXAMPLE_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[rstest]
    #[case(INPUT_EXAMPLE_1, "2")]
    #[case(INPUT_EXAMPLE_2, "6")]
    fn process_with_example(#[case] input: &str, #[case] expected: String) {
        let result = process(input);
        assert_eq!(result, expected);
    }
}
