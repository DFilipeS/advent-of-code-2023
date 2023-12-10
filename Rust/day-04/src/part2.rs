use std::collections::BTreeMap;

use crate::part1::parse_input;

pub fn process(input: &str) -> String {
    let (_, cards) = parse_input(input).expect("should parse input");
    let mut instances: BTreeMap<u32, u32> = BTreeMap::new();

    for card in cards {
        // Add the (current) instance of the card to the record and get the
        // total amount of copies of the current card that we have.
        let copies = *instances
            .entry(card.id)
            .and_modify(|val| *val += 1)
            .or_insert(1);

        // For each winning number, we add the number of copies of the current
        // card of the new card to the record.
        for id in 0..card.count_winners() {
            let id = card.id + 1 + id;
            instances
                .entry(id)
                .and_modify(|val| *val += copies)
                .or_insert(copies);
        }
    }

    instances.values().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn process_with_example_input() {
        let result = process(INPUT);
        assert_eq!(result, "30");
    }
}
