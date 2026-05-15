use crate::days::Solution;

pub struct Day04;

fn get_score(line: &str) -> u64 {
    let mut score = 0;
    if let Some((_, numbers)) = line.split_once(":") {
        if let Some((winning, mynumbers)) = numbers.split_once("|") {
            let winning_values = winning
                .split_whitespace()
                .map(|s| s.parse::<u64>().expect("Should be a digit"))
                .collect::<Vec<u64>>();
            let mynumbers_values = mynumbers
                .split_whitespace()
                .map(|s| s.parse::<u64>().expect("Should be a digit"))
                .collect::<Vec<u64>>();
            for w in winning_values {
                for m in &mynumbers_values {
                    if w == *m {
                        if score == 0 {
                            score = 1;
                        } else {
                            score *= 2;
                        }
                    }
                }
            }
        }
    }
    score
}

impl Solution for Day04 {
    fn part1(input: &str) -> String {
        let mut res = 0;
        for line in input.split('\n') {
            res += get_score(line);
        }
        format!("part1: {res}")
    }
    fn part2(input: &str) -> String {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let mut res = 0;
        let mut scratchcards = Vec::<(&str, u64)>::new();
        let mut next = 1;
        for line in input.split('\n') {
            res += 1;
            let mut score = get_score(line);
            if score == 0 {
                scratchcards.push((line, next));
                next = 1;
            } else {
                scratchcards.push((line, next));
                next = 1;
                while score > 1 {
                    score /= 2;
                    next += 1;
                }
            }
        }
        res += scratchcards.iter().map(|(_, n)| n).sum::<u64>();
        format!("part2: {res}")
    }
}
