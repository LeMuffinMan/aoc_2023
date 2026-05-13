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
        let mut res: u64 = 0;
        let grid: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect();
        let height = grid.len();
        let width = grid[0].len();

        for y in 0..height {
            for x in 0..width {
                if grid[y][x] != '*' {
                    continue;
                }

                let mut part_numbers: Vec<u64> = Vec::new();

                for dy in [-1i32, 0, 1] {
                    let ny = y as i32 + dy;
                    if ny < 0 || ny >= height as i32 {
                        continue;
                    }
                    let ny = ny as usize;

                    if dy == 0 {
                        if x > 0 && grid[ny][x - 1].is_ascii_digit() {
                            part_numbers.push(get_number(&grid, ny, x - 1));
                        }
                        if x + 1 < width && grid[ny][x + 1].is_ascii_digit() {
                            part_numbers.push(get_number(&grid, ny, x + 1));
                        }
                    } else {
                        let x_start = if x > 0 { x - 1 } else { 0 };
                        let x_end = (x + 1).min(width - 1);
                        let mut cx = x_start;
                        while cx <= x_end {
                            if grid[ny][cx].is_ascii_digit() {
                                part_numbers.push(get_number(&grid, ny, cx));
                                while cx <= x_end && grid[ny][cx].is_ascii_digit() {
                                    cx += 1;
                                }
                            } else {
                                cx += 1;
                            }
                        }
                    }
                }

                if part_numbers.len() == 2 {
                    res += part_numbers[0] * part_numbers[1];
                }
            }
        }

        format!("part2: {res}")
    }
}

fn get_number(grid: &Vec<Vec<char>>, y: usize, x: usize) -> u64 {
    let mut start = x;
    while start > 0 && grid[y][start - 1].is_ascii_digit() {
        start -= 1;
    }
    let mut end = start;
    while end < grid[y].len() && grid[y][end].is_ascii_digit() {
        end += 1;
    }
    grid[y][start..end]
        .iter()
        .collect::<String>()
        .parse()
        .unwrap()
}
