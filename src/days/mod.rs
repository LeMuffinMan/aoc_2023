pub mod day01;
pub mod day02;

pub trait Solution {
    fn part1(input: &str);
    fn part2(input: &str);
}

pub fn solve(day: u8, part: Option<u8>, input: String) {
    println!("Day{day}");
    match day {
        1 => dispatch::<day01::Day01>(part, &input),
        2 => dispatch::<day02::Day02>(part, &input),
        _ => eprintln!("Day {} not implemented yet", day),
    }
}

fn dispatch<D: Solution>(part: Option<u8>, input: &str) {
    match part {
        Some(1) => D::part1(input),
        Some(2) => D::part2(input),
        None => {
            D::part1(input);
            D::part2(input);
        }
        Some(p) => eprintln!("Unknown part {}", p),
    }
}
