use crate::days::Solution;

pub struct Day02;

fn is_possible(line: &str) -> bool {
    if let Some((_, gameinfo)) = line.split_once(':') {
        for set in gameinfo.split(';') {
            for cube in set.split(',') {
                if let Some((value, color)) = cube.trim().split_once(' ') {
                    match color {
                        "red" => {
                            if let Ok(n) = value.parse::<u64>() {
                                if n > 12 {
                                    return false;
                                }
                            }
                        }
                        "green" => {
                            if let Ok(n) = value.parse::<u64>() {
                                if n > 13 {
                                    return false;
                                }
                            }
                        }
                        "blue" => {
                            if let Ok(n) = value.parse::<u64>() {
                                if n > 14 {
                                    return false;
                                }
                            }
                        }
                        _ => unreachable!("found unexpected color: {color}"),
                    }
                }
            }
        }
    } else {
        return false;
    }
    true
}

fn power(line: &str) -> u64 {
    let mut min_r = 0;
    let mut min_g = 0;
    let mut min_b = 0;

    if let Some((_, gameinfo)) = line.split_once(':') {
        for set in gameinfo.split(';') {
            for cube in set.split(',') {
                if let Some((value, color)) = cube.trim().split_once(' ') {
                    match color {
                        "red" => min_r = std::cmp::max(min_r, value.parse::<u64>().unwrap_or(0)),
                        "green" => min_g = std::cmp::max(min_g, value.parse::<u64>().unwrap_or(0)),
                        "blue" => min_b = std::cmp::max(min_b, value.parse::<u64>().unwrap_or(0)),
                        _ => unreachable!("found unexpected color: {color}"),
                    };
                }
            }
        }
    }
    min_r * min_g * min_b
}

impl Solution for Day02 {
    fn part1(input: &str) -> String {
        let mut id = 1;
        let mut res = 0;
        for l in input.split('\n') {
            if is_possible(l) {
                res += id;
            }
            id += 1;
        }
        format!("part1: {res}")
    }
    fn part2(input: &str) -> String {
        let res: u64 = input.split('\n').map(|l| power(l)).sum();
        format!("part2: {res}")
    }
}
