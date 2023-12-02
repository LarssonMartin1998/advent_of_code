use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::Chars;

fn get_first_num_in_string(input: &str) -> char {
    get_first_number_in_iterator(input.chars())
}

fn get_last_num_in_string(input: &str) -> char {
    let char_vec: String = input.chars().rev().collect();
    get_first_number_in_iterator(char_vec.chars())
}

fn get_first_number_in_iterator(input: Chars) -> char {
    for c in input {
        if c.is_numeric() {
            return c;
        }
    }

    '0'
}

fn get_calibration_value(input: &str) -> String {
    let string_literal = [
        get_first_num_in_string(input),
        get_last_num_in_string(input),
    ]
    .iter()
    .collect::<String>();

    string_literal
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut total = 0;
    for line in reader.lines() {
        let result = get_calibration_value(&line?);
        let parsed: i32 = result.parse().unwrap();
        total += parsed;
    }

    println!("{}", total);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_solution() {
        let test_cases = [
            ("1abc2", "12"),
            ("pqr3stu8vwx", "38"),
            ("a1b2c3d4e5f", "15"),
            ("treb7uchet", "77"),
        ];

        for (input, expected_result) in test_cases {
            let result = get_calibration_value(input);
            assert_eq!(result, expected_result);
        }
    }
}
