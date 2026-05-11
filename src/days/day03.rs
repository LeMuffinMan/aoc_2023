use crate::days::Solution;

pub struct Day03;

impl Solution for Day03 {
    fn part1(input: &str) -> String {
        let mut res = 0;
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let height = grid.len();
        let width = grid[0].len();

        for (y, row) in grid.iter().enumerate() {
            let mut x = 0;
            while x < row.len() {
                if row[x].is_ascii_digit() {
                    let x_start = x;
                    while x < row.len() && row[x].is_ascii_digit() {
                        x += 1;
                    }
                    let x_end = x;
                    let x_min = x_start.saturating_sub(1);
                    let x_max = (x + 1).min(width);
                    let y_min = y.saturating_sub(1);
                    let y_max = (y + 1).min(height - 1);

                    let has_symbol = (y_min..=y_max).any(|ny| {
                        (x_min..x_max).any(|nx| {
                            ny < height && nx < width && {
                                let c = grid[ny][nx];
                                !c.is_ascii_digit() && c != '.'
                            }
                        })
                    });

                    if has_symbol {
                        if let Ok(n) = row[x_start..x_end]
                            .iter()
                            .collect::<String>()
                            .parse::<u64>()
                        {
                            res += n;
                        }
                    }
                } else {
                    x += 1;
                }
            }
        }
        format!("part1: {res}")
    }
    fn part2(input: &str) -> String {
        let mut res = 0;
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";
        format!("part2: {res}")
    }
}
