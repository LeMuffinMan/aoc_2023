pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

pub trait Solution {
    fn part1(input: &str) -> String;
    fn part2(input: &str) -> String;
}

pub fn solve(day: u8, part: Option<u8>, input: String) -> String {
    println!("Day{day}");
    match day {
        1 => dispatch::<day01::Day01>(part, &input),
        2 => dispatch::<day02::Day02>(part, &input),
        3 => dispatch::<day03::Day03>(part, &input),
        4 => dispatch::<day04::Day04>(part, &input),
        _ => format!("Day {} not implemented yet", day),
    }
}

fn dispatch<D: Solution>(part: Option<u8>, input: &str) -> String {
    match part {
        Some(1) => D::part1(input),
        Some(2) => D::part2(input),
        None => format!("{}\n{}", D::part1(input), D::part2(input)),
        Some(p) => format!("Unknown part {}", p),
    }
}
