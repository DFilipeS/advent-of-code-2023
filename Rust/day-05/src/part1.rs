use std::ops::Range;

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

#[derive(Debug)]
pub struct Mapping(Range<u64>, Range<u64>);

#[derive(Debug)]
pub struct Mapper {
    pub ranges: Vec<Mapping>,
}

impl Mapper {
    pub fn look_up(&self, value: u64) -> u64 {
        for mapping in self.ranges.iter() {
            if mapping.0.contains(&value) {
                let offset = value - mapping.0.start;
                return mapping.1.start + offset;
            }
        }
        value
    }
}

#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<u64>,
    pub seed_to_soil: Mapper,
    pub soil_to_fertilizer: Mapper,
    pub fertilizer_to_water: Mapper,
    pub water_to_light: Mapper,
    pub light_to_temperature: Mapper,
    pub temperature_to_humidity: Mapper,
    pub humidity_to_location: Mapper,
}

impl Almanac {
    pub fn find_location(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil.look_up(seed);
        let fertilizer = self.soil_to_fertilizer.look_up(soil);
        let water = self.fertilizer_to_water.look_up(fertilizer);
        let light = self.water_to_light.look_up(water);
        let temperature = self.light_to_temperature.look_up(light);
        let humidity = self.temperature_to_humidity.look_up(temperature);

        self.humidity_to_location.look_up(humidity)
    }
}

pub fn process(input: &str) -> String {
    let (_, almanac) = parse_input(input).expect("should parse input");

    almanac
        .seeds
        .iter()
        .map(|seed| almanac.find_location(*seed))
        .min()
        .expect("should exist minimum location value")
        .to_string()
}

pub fn parse_input(input: &str) -> IResult<&str, Almanac> {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let seeds_section = sections.first().expect("should have section");
    let seed_to_soil_map_section = sections.get(1).expect("should have section");
    let soil_to_fertilizer_map_section = sections.get(2).expect("should have section");
    let fertilizer_to_water_map_section = sections.get(3).expect("should have section");
    let water_to_light_map_section = sections.get(4).expect("should have section");
    let light_to_temperature_map_section = sections.get(5).expect("should have section");
    let temperature_to_humidity_map_section = sections.get(6).expect("should have section");
    let humidity_to_location_map_section = sections.get(7).expect("should have section");
    let (_, seeds) = parse_seeds(seeds_section)?;
    let (_, seed_to_soil) = parse_seed_to_soil_map(seed_to_soil_map_section)?;
    let (_, soil_to_fertilizer) = parse_soil_to_fertilizer_map(soil_to_fertilizer_map_section)?;
    let (_, fertilizer_to_water) = parse_fertilizer_to_water_map(fertilizer_to_water_map_section)?;
    let (_, water_to_light) = parse_water_to_light_map(water_to_light_map_section)?;
    let (_, light_to_temperature) =
        parse_light_to_temperature_map(light_to_temperature_map_section)?;
    let (_, temperature_to_humidity) =
        parse_temperature_to_humidity_map(temperature_to_humidity_map_section)?;
    let (_, humidity_to_location) =
        parse_humidity_to_location_map(humidity_to_location_map_section)?;

    let almanac = Almanac {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    };

    Ok(("", almanac))
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(tag("seeds: "), separated_list1(space1, complete::u64))(input)
}

fn parse_seed_to_soil_map(input: &str) -> IResult<&str, Mapper> {
    let (input, _) = tag("seed-to-soil map:\n")(input)?;
    let (input, ranges) = parse_mappings(input)?;

    Ok((input, Mapper { ranges }))
}

fn parse_soil_to_fertilizer_map(input: &str) -> IResult<&str, Mapper> {
    let (input, _) = tag("soil-to-fertilizer map:\n")(input)?;
    let (input, ranges) = parse_mappings(input)?;

    Ok((input, Mapper { ranges }))
}

fn parse_fertilizer_to_water_map(input: &str) -> IResult<&str, Mapper> {
    let (input, _) = tag("fertilizer-to-water map:\n")(input)?;
    let (input, ranges) = parse_mappings(input)?;

    Ok((input, Mapper { ranges }))
}

fn parse_water_to_light_map(input: &str) -> IResult<&str, Mapper> {
    let (input, _) = tag("water-to-light map:\n")(input)?;
    let (input, ranges) = parse_mappings(input)?;

    Ok((input, Mapper { ranges }))
}

fn parse_light_to_temperature_map(input: &str) -> IResult<&str, Mapper> {
    let (input, _) = tag("light-to-temperature map:\n")(input)?;
    let (input, ranges) = parse_mappings(input)?;

    Ok((input, Mapper { ranges }))
}

fn parse_temperature_to_humidity_map(input: &str) -> IResult<&str, Mapper> {
    let (input, _) = tag("temperature-to-humidity map:\n")(input)?;
    let (input, ranges) = parse_mappings(input)?;

    Ok((input, Mapper { ranges }))
}

fn parse_humidity_to_location_map(input: &str) -> IResult<&str, Mapper> {
    let (input, _) = tag("humidity-to-location map:\n")(input)?;
    let (input, ranges) = parse_mappings(input)?;

    Ok((input, Mapper { ranges }))
}

fn parse_mappings(input: &str) -> IResult<&str, Vec<Mapping>> {
    separated_list1(line_ending, |input| {
        let (input, dest) = complete::u64(input)?;
        let (input, _) = space1(input)?;
        let (input, source) = complete::u64(input)?;
        let (input, _) = space1(input)?;
        let (input, length) = complete::u64(input)?;

        Ok((input, Mapping(source..source + length, dest..dest + length)))
    })(input)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

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
        assert_eq!(result, "35");
    }

    #[rstest]
    #[case(79, 81)]
    #[case(14, 14)]
    #[case(55, 57)]
    #[case(13, 13)]
    fn mapper_look_up_with_example(#[case] seed: u64, #[case] expected: u64) {
        let mapper = Mapper {
            ranges: vec![Mapping(98..100, 50..52), Mapping(50..98, 52..100)],
        };
        let result = mapper.look_up(seed);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(79, 82)]
    #[case(14, 43)]
    #[case(55, 86)]
    #[case(13, 35)]
    fn find_location_with_example_input(#[case] seed: u64, #[case] expected: u64) {
        let (_, almanac) = parse_input(INPUT).unwrap();
        let result = almanac.find_location(seed);
        assert_eq!(result, expected);
    }
}
