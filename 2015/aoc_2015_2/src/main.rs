// https://adventofcode.com/2015/day/2

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::AddAssign;

struct Packaging {
    paper: u32,
    ribbon: u32,
}

impl Default for Packaging {
    fn default() -> Self {
        Self {
            paper: 0,
            ribbon: 0,
        }
    }
}

impl AddAssign<&Packaging> for Packaging {
    fn add_assign(&mut self, rhs: &Self) {
        self.paper += rhs.paper;
        self.ribbon += rhs.ribbon;
    }
}

struct Gift {
    packaging: Packaging,
}

impl Gift {
    fn new(w: u32, l: u32, h: u32) -> Self {
        let mut dimensions = [w, l, h];
        dimensions.sort();

        let packaging = Packaging {
            paper: Self::get_required_wrapping_paper(&dimensions),
            ribbon: Self::get_required_ribbon(&dimensions),
        };

        Self { packaging }
    }

    fn new_arr(dimensions: &[u32; 3]) -> Self {
        Self::new(dimensions[0], dimensions[1], dimensions[2])
    }

    const fn get_required_wrapping_paper(dimensions: &[u32; 3]) -> u32 {
        let slack = dimensions[0] * dimensions[1];
        let packaging_surface_area =
            get_cuboid_surface_area(dimensions[0], dimensions[1], dimensions[2]);
        packaging_surface_area + slack
    }

    fn get_required_ribbon(sorted_dimensions: &[u32; 3]) -> u32 {
        let wrapping = 2 * (sorted_dimensions[0] + sorted_dimensions[1]);
        let bow = sorted_dimensions.iter().fold(1, |acc, x| acc * x);

        wrapping + bow
    }

    const fn get_required_packaging(&self) -> &Packaging {
        &self.packaging
    }
}

const fn get_rectangle_area(w: u32, l: u32) -> u32 {
    w * l
}

const fn get_cuboid_surface_area(w: u32, l: u32, h: u32) -> u32 {
    2 * (get_rectangle_area(w, l) + get_rectangle_area(w, h) + get_rectangle_area(h, l))
}

fn create_gifts_from_input(gifts: &mut Vec<Gift>) {
    let file = File::open("input.txt").expect("input.txt is expected to be in the src directory.");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let split_line: Vec<&str> = line.split('x').collect();
        let mut dimensions: [u32; 3] = [0; 3];
        for (i, slice) in split_line.iter().enumerate() {
            dimensions[i] = slice.parse().unwrap();
        }

        gifts.push(Gift::new_arr(&dimensions));
    }
}

fn main() {
    let mut gifts = Vec::new();
    create_gifts_from_input(&mut gifts);

    let mut total_packaging = Packaging::default();
    for gift in &gifts {
        total_packaging += gift.get_required_packaging();
    }

    println!(
        "Packaging=[Paper: {}], [Ribbon: {}]",
        total_packaging.paper, total_packaging.ribbon
    );
}
