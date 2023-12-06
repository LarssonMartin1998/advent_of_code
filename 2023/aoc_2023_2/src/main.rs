use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

struct Game {
    cube_sets: Vec<CubeSet>,
}
struct CubeSet {
    cubes: Vec<Cube>,
}

enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

fn parse_input() -> Result<Vec<Game>, io::Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    let mut games: Vec<Game> = Vec::new();
    for line in reader.lines() {
        games.push(parse_line(&line.unwrap())?);
    }

    Ok(games)
}

fn parse_line(line: &str) -> Result<Game, io::Error> {
    // discard unnecessary Game x:
    let cleaned_line = line.split(":").last().unwrap();
    let game = cleaned_line.split(";");
    let mut cube_sets: Vec<CubeSet> = Vec::new();

    for rounds in game {
        let cubes = rounds.split(",");
        let mut cube_set: Vec<Cube> = Vec::new();
        for cube in cubes {
            cube_set.push(parse_cube(cube)?);
        }

        cube_sets.push(CubeSet { cubes: cube_set });
    }

    Ok(Game { cube_sets })
}

fn parse_cube(cube_text: &str) -> Result<Cube, io::Error> {
    let cleaned_cube = &cube_text[1..];
    let mut split_cube = cleaned_cube.split(" ");
    let num_cubes: u32 = split_cube.next().unwrap().parse().unwrap();
    let cube_type_text = split_cube.last().unwrap();

    match cube_type_text {
        "red" => Ok(Cube::Red(num_cubes)),
        "green" => Ok(Cube::Green(num_cubes)),
        "blue" => Ok(Cube::Blue(num_cubes)),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "cube text doesn't match any cube type",
        )),
    }
}

fn main() -> io::Result<()> {
    let games = parse_input()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Cube {
        fn get_count(&self) -> u32 {
            match self {
                Cube::Red(value) => *value,
                Cube::Green(value) => *value,
                Cube::Blue(value) => *value,
            }
        }

        fn get_name(&self) -> &str {
            match self {
                Cube::Red(_) => "red",
                Cube::Green(_) => "green",
                Cube::Blue(_) => "blue",
            }
        }
    }

    fn reconstruct_input(games: &Vec<Game>) -> Vec<String> {
        let mut reconstructed_input: Vec<String> = Vec::new();

        for (i, game) in games.iter().enumerate() {
            let mut game_string = "Game ".to_owned();
            game_string.push_str(&(i + 1).to_string());
            game_string.push_str(":");

            for cube_set in &game.cube_sets {
                for cube in &cube_set.cubes {
                    game_string.push_str(" ");
                    game_string.push_str(&cube.get_count().to_string());
                    game_string.push_str(" ");
                    game_string.push_str(cube.get_name());
                    game_string.push_str(",");
                }

                game_string.pop();
                game_string.push_str(";");
            }

            game_string.pop();
            reconstructed_input.push(game_string);
        }

        reconstructed_input
    }

    #[test]
    fn test() {
        let input = File::open("input.txt").unwrap();
        let reader = BufReader::new(input);

        let games = parse_input().unwrap();
        let reconstructed_input = reconstruct_input(&games);
        for (i, line) in reader.lines().enumerate() {
            assert_eq!(reconstructed_input[i], line.unwrap());
        }
    }
}
