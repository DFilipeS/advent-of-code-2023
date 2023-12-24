use crate::part1::{parse_input, Instruction};

pub fn process(input: &str) -> String {
    let (_, (instructions, nodes)) = parse_input(input).unwrap();

    nodes
        .keys()
        .filter(|key| key.ends_with('A'))
        .map(|mut cursor| {
            let mut cursor_index = 0;

            loop {
                if cursor.ends_with('Z') {
                    break;
                }
                let node = nodes.get(cursor).expect("node should exist");
                let instruction = instructions
                    .get(cursor_index % instructions.len())
                    .expect("instruction should exist");

                cursor = match instruction {
                    Instruction::Left => &node.0,
                    Instruction::Right => &node.1,
                };

                cursor_index += 1;
            }

            cursor_index
        })
        .reduce(least_common_multiple)
        .expect("should have a result")
        .to_string()
}

fn least_common_multiple(a: usize, b: usize) -> usize {
    (a * b) / greatest_common_divisor(a, b)
}

fn greatest_common_divisor(mut a: usize, mut b: usize) -> usize {
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    a
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    const INPUT: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[rstest]
    #[case(INPUT, "6")]
    fn process_with_example(#[case] input: &str, #[case] expected: String) {
        let result = process(input);
        assert_eq!(result, expected);
    }
}
