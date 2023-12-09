use itertools::Itertools;

pub fn process(input: &str) -> String {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut part_numbers: Vec<u32> = Vec::new();

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

                if is_part_number(&matrix, &digits, (row, col)) {
                    let part_number = digits.parse().expect("should be a valid number");
                    part_numbers.push(part_number);
                }
            }
        }
    }

    part_numbers.iter().sum::<u32>().to_string()
}

pub fn read_digits(values: &[char], element: char) -> String {
    let mut digits = String::new();
    digits.push(element);

    // Collect the remaining digits of the number and advance the
    // iterator if more digits are found.
    loop {
        let offset = digits.len();
        match values.get(offset) {
            Some(value) => {
                if !value.is_ascii_digit() {
                    break;
                }
                digits.push(*value);
            }
            None => break,
        }
    }
    digits
}

fn is_part_number(matrix: &[Vec<char>], digits: &str, (row, col): (usize, usize)) -> bool {
    // Check surrounding positions in the matrix for any symbol
    // different from '.' and an ASCII digit.
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
                if !value.is_ascii_digit() && *value != '.' {
                    return true;
                }
            }
        }
    }

    false
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
        assert_eq!(result, "4361");
    }
}
