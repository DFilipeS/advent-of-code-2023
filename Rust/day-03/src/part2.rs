use std::collections::HashMap;

use itertools::Itertools;

use crate::part1::read_digits;

pub fn process(input: &str) -> String {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut gear_ratios: HashMap<_, Vec<usize>> = HashMap::new();

    for (row, values) in matrix.iter().enumerate() {
        let mut row_iter = values.iter().enumerate();

        while let Some((col, element)) = row_iter.next() {
            // Check if current character is a ASCII digit.
            if element.is_ascii_digit() {
                // Read all the number digits.
                let digits = read_digits(&values[col..], *element);
                for _ in 0..digits.len() {
                    row_iter.next();
                }

                if let Some(position) = adjacent_gear(&matrix, &digits, (row, col)) {
                    let number = digits.parse().expect("should be a valid number");
                    gear_ratios.entry(position).or_default().push(number);
                }
            }
        }
    }

    gear_ratios
        .iter()
        .filter_map(|(_, part_numbers)| {
            // We just want gears that have two parts adjacent.
            if part_numbers.len() > 1 {
                return Some(part_numbers.iter().product::<usize>());
            }
            None
        })
        .sum::<usize>()
        .to_string()
}

fn adjacent_gear(
    matrix: &[Vec<char>],
    digits: &str,
    (row, col): (usize, usize),
) -> Option<(usize, usize)> {
    // Check surrounding positions in the matrix for a '*' symbol.
    let col_range = if col == 0 {
        0..=digits.len()
    } else {
        col - 1..=(col + digits.len())
    };

    let row_range = if row == 0 {
        0..=row + 1
    } else {
        row - 1..=(row + 1)
    };

    for (r, c) in row_range.cartesian_product(col_range) {
        if let Some(row) = matrix.get(r) {
            if let Some(value) = row.get(c) {
                if *value == '*' {
                    return Some((r, c));
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn process_with_example_input() {
        let result = process(INPUT);
        assert_eq!(result, "467835");
    }
}
