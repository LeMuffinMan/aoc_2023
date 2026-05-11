use crate::days::Solution;

pub struct Day01;

fn get_calibration_value_part1(line: &str) -> u64 {
    let mut res = 0;
    for c in line.chars().filter_map(|c| c.to_digit(10)) {
        res = c as u64 * 10;
        break;
    }
    for c in line.chars().rev().filter_map(|c| c.to_digit(10)) {
        res += c as u64;
        break;
    }
    res
}

fn get_digit(line: &str, i: usize) -> Option<u64> {
    if line[i..].starts_with("zero") {
        Some(0)
    } else if line[i..].starts_with("one") {
        Some(1)
    } else if line[i..].starts_with("two") {
        Some(2)
    } else if line[i..].starts_with("three") {
        Some(3)
    } else if line[i..].starts_with("four") {
        Some(4)
    } else if line[i..].starts_with("five") {
        Some(5)
    } else if line[i..].starts_with("six") {
        Some(6)
    } else if line[i..].starts_with("seven") {
        Some(7)
    } else if line[i..].starts_with("eight") {
        Some(8)
    } else if line[i..].starts_with("nine") {
        Some(9)
    } else if line.as_bytes()[i].is_ascii_digit() {
        Some((line.as_bytes()[i] - b'0') as u64)
    } else {
        None
    }
}

fn get_calibration_value_part2(line: &str) -> u64 {
    let mut res = 0;
    for i in 0..line.len() {
        if let Some(digit) = get_digit(line, i) {
            res = digit * 10;
            break;
        }
    }
    for i in (0..line.len()).rev() {
        if let Some(digit) = get_digit(line, i) {
            res += digit;
            break;
        }
    }
    res
}

impl Solution for Day01 {
    fn part1(input: &str) -> String {
        let res: u64 = input
            .split('\n')
            .map(|l| get_calibration_value_part1(l))
            .sum();
        format!("part1: {res}")
    }
    fn part2(input: &str) -> String {
        let res: u64 = input
            .split('\n')
            .map(|l| get_calibration_value_part2(l))
            .sum();
        format!("part2: {res}")
    }
}
