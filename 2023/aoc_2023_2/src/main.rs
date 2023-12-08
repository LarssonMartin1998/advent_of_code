use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Clone)]
struct Game {
    id: u32,
    cube_sets: Vec<CubeSet>,
}

#[derive(Clone)]
struct CubeSet {
    cubes: Vec<Cube>,
}

#[derive(Clone)]
enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

trait CubeComparison {
    fn is_same_type(&self, other: &Self) -> bool;
}

impl CubeComparison for Cube {
    fn is_same_type(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl Cube {
    fn get_count(&self) -> Option<u32> {
        match self {
            Cube::Red(value) | Cube::Green(value) | Cube::Blue(value) => Some(*value),
        }
    }
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
    let mut id_separator = line.split(":");
    let id_string = id_separator.next().unwrap();
    let id: u32 = id_string.split("Game ").last().unwrap().parse().unwrap();

    let cleaned_line = id_separator.last().unwrap();
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

    Ok(Game { id, cube_sets })
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

fn remove_invalid_games(games: &mut Vec<Game>) {
    let invalid_indices: Vec<usize> = games
        .iter()
        .enumerate()
        .filter_map(|(i, game)| {
            if game
                .cube_sets
                .iter()
                .any(|cube_set| !is_cube_set_valid(&cube_set))
            {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    for &i in invalid_indices.iter().rev() {
        games.remove(i);
    }
}

fn is_cube_set_valid(cube_set: &CubeSet) -> bool {
    let limits = [Cube::Red(12), Cube::Green(13), Cube::Blue(14)];
    for cube in &cube_set.cubes {
        if limits
            .iter()
            .any(|limit| cube.is_same_type(&limit) && cube.get_count() > limit.get_count())
        {
            return false;
        }
    }

    true
}

fn part_one(games: &Vec<Game>) -> u32 {
    let mut games_copy = games.to_vec();
    remove_invalid_games(&mut games_copy);
    games_copy.iter().fold(0, |acc, game| acc + game.id)
}

fn part_two(games: &Vec<Game>) -> u32 {
    games.iter().fold(0, |acc, game| {
        let mut highest: [Cube; 3] = [Cube::Red(0), Cube::Green(0), Cube::Blue(0)];

        for cube_set in &game.cube_sets {
            for cube in &cube_set.cubes {
                try_update_highest_of_type(&mut highest, &cube);
            }
        }

        acc + highest
            .iter()
            .fold(1, |acc, cube| acc * cube.get_count().unwrap_or_default())
    })
}

fn try_update_highest_of_type(current_highest: &mut [Cube; 3], other: &Cube) {
    for cube in current_highest
        .iter_mut()
        .filter(|cube| cube.is_same_type(other) && other.get_count() > cube.get_count())
    {
        *cube = other.clone();
    }
}

fn main() -> io::Result<()> {
    let games = parse_input()?;
    println!("Results for part one: {}", part_one(&games));
    println!("Results for part two: {}", part_two(&games));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Cube {
        fn get_name(&self) -> &str {
            match self {
                Cube::Red(_) => "red",
                Cube::Green(_) => "green",
                Cube::Blue(_) => "blue",
            }
        }
    }

    fn reconstruct_input(games: &Vec<Game>) -> Vec<String> {
        games
            .iter()
            .map(|game| {
                let mut game_string = format!("Game {}:", game.id);
                for cube_set in &game.cube_sets {
                    for cube in &cube_set.cubes {
                        game_string += &format!(
                            " {} {},",
                            cube.get_count().unwrap_or_default(),
                            cube.get_name()
                        );
                    }

                    game_string.pop();
                    game_string += ";";
                }

                game_string.pop();
                game_string
            })
            .collect()
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
