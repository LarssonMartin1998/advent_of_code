use std::fs::File;
use std::i32;
use std::io::{self, prelude::*, BufReader};
use std::str::Chars;

fn get_first_num_in_string(input: &str) -> Result<i32, io::Error> {
    let number_strings = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    get_first_number_in_iterator(input.chars(), &number_strings)
}

fn get_last_num_in_string(input: &str) -> Result<i32, io::Error> {
    let number_strings = [
        ("eno", 1),
        ("owt", 2),
        ("eerht", 3),
        ("ruof", 4),
        ("evif", 5),
        ("xis", 6),
        ("neves", 7),
        ("thgie", 8),
        ("enin", 9),
    ];

    let char_vec: String = input.chars().rev().collect();
    get_first_number_in_iterator(char_vec.chars(), &number_strings)
}

fn get_first_number_in_iterator(
    input: Chars,
    number_strings: &[(&str, i32)],
) -> Result<i32, io::Error> {
    let mut curr_number_string = "".to_owned();
    for c in input {
        if c.is_numeric() {
            return Ok(c.to_string().parse().unwrap());
        }

        curr_number_string.push(c);
        if curr_number_string.len() > 5 {
            let mut chars = curr_number_string.chars();
            chars.next();
            curr_number_string = chars.as_str().to_string();
        }

        for (number_name, number_value) in number_strings {
            if curr_number_string.contains(number_name) {
                return Ok(*number_value);
            }
        }
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "Iterator doesn't contain a number.",
    ))
}

fn get_calibration_value(input: &str) -> Result<i32, io::Error> {
    Ok(get_first_num_in_string(input)? * 10 + get_last_num_in_string(input)?)
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut total = 0;
    for line in reader.lines() {
        let result = get_calibration_value(&line?)?;
        total += result;
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
            ("two1nine", 29),
            ("eighttwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwoone3four", 24),
        ];

        for (input, expected_result) in test_cases {
            let result = get_calibration_value(input).unwrap();
            assert_eq!(result, expected_result);
        }
    }
}
