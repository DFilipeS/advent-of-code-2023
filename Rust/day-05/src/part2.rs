use crate::part1::parse_input;

pub fn process(input: &str) -> String {
    let (_, almanac) = parse_input(input).expect("should parse input");

    almanac
        .seeds
        .chunks_exact(2)
        .map(|chunk| {
            let start = chunk.first().expect("should have start value");
            let length = chunk.last().expect("should have length value");
            (*start..(*start + *length))
                .map(|seed| almanac.find_location(seed))
                .min()
                .expect("should exist minimum location value")
        })
        .min()
        .expect("should exist minimum location value")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn process_with_example_input() {
        let result = process(INPUT);
        assert_eq!(result, "46");
    }
}
