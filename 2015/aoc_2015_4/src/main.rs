fn build_hash_input(original_string: &str, count: i32) -> String {
    if count == 0 {
        return original_string.to_owned();
    }

    format!("{}{}", original_string, count)
}

fn build_hash_match(num_zeroes: usize) -> String {
    std::iter::repeat('0').take(num_zeroes).collect()
}

fn process_input(input: (&str, usize)) -> String {
    let hash_match = build_hash_match(input.1);
    let mut i = 0;
    loop {
        let hash_input = build_hash_input(input.0, i);
        let hash_result = format!("{:x}", md5::compute(hash_input.clone()));
        if hash_result[..input.1] == hash_match.to_owned() {
            return i.to_string();
        }

        if i > 100000000 {
            return "max iterations".to_owned();
        }

        i += 1;
    }
}

fn main() {
    let inputs = [("abcdef", 5), ("pqrstuv", 5), ("bgvyzdsv", 6)];
    for input in inputs {
        println!("{}", process_input(input));
    }
}

#[cfg(test)]
mod tests {
    struct TestInput<'a> {
        input: (&'a str, usize),
        expected: &'a str,
    }

    impl<'a> TestInput<'a> {
        fn new(input: &'a str, match_size: usize, expected: &'a str) -> Self {
            TestInput {
                input: (input, match_size),
                expected,
            }
        }
    }

    #[test]
    fn test_input() {
        let inputs = [
            TestInput::new("abcdef", 5, "609043"),
            TestInput::new("pqrstuv", 5, "1048970"),
        ];

        for input in inputs {
            assert_eq!(super::process_input(input.input), input.expected);
        }
    }
}
