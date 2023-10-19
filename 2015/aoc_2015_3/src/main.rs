use std::{collections::HashMap, fs};

type Coordinates = (i32, i32);
type HouseMap = HashMap<Coordinates, u32>;

fn update_coords(c: char, coords: &mut Coordinates) {
    match c {
        '^' => coords.1 += 1,
        '<' => coords.0 += -1,
        'v' => coords.1 += -1,
        '>' => coords.0 += 1,
        _ => return,
    }
}

fn deliver_gift(coords: &Coordinates, houses: &mut HouseMap) {
    let value = houses.entry(*coords).or_insert(0);
    *value += 1;
}

fn process_input(input: &String) -> u32 {
    let mut santa = (0, 0);
    let mut robo_santa = (0, 0);
    let mut houses = HouseMap::new();
    houses.insert((0, 0), 2); // Insert santas starting position
    for (i, c) in input.chars().enumerate() {
        let active_santa = if i % 2 == 0 {
            &mut santa
        } else {
            &mut robo_santa
        };

        update_coords(c, active_santa);
        deliver_gift(active_santa, &mut houses)
    }

    houses.len() as u32
}

fn main() {
    let filepath = "input.txt";
    let input = fs::read_to_string(filepath).unwrap();
    let result = process_input(&input);

    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_santa() {
        let tests = [("^v", 3), ("^>v<", 3), ("^v^v^v^v^v", 11)];
        for test in tests {
            assert_eq!(super::process_input(&test.0.to_owned()), test.1);
        }
    }
}
