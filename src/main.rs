use aoc_2023::days::solve;
use clap::Parser;
use reqwest::blocking::Client;
use std::env;
use std::error::Error;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    day: Option<u8>,
    #[arg(short, long)]
    part: Option<u8>,
}

const DAYS: u8 = 2;

pub fn fetch(day: u8) -> Result<String, Box<dyn Error>> {
    dotenv::from_path(".env").ok();
    let session_cookie = env::var("AOC_SESSION")?;

    let url = format!("https://adventofcode.com/2023/day/{day}/input");

    let client = Client::new();
    let response = client
        .get(url)
        .header("Cookie", format!("session={}", session_cookie))
        .send()?
        .text()?;

    Ok(response)
}

fn get_input(day: u8) -> Result<String, Box<dyn Error>> {
    let path = format!("inputs/day{:02}.txt", day);
    let input = match Path::new(&path).exists() {
        true => std::fs::read_to_string(format!("inputs/day{:02}.txt", day))?,
        false => {
            std::fs::create_dir_all("inputs")?;
            let content = fetch(day)?;
            std::fs::write(&path, &content)?;
            content
        }
    };
    Ok(input)
}

fn main() {
    let args = Cli::parse();
    match args.day {
        Some(day) => match get_input(day) {
            Ok(input) => println!("{}", solve(day, args.part, input)),
            Err(e) => eprintln!("Error: {e}"),
        },
        None => {
            for day in 1..=DAYS {
                match get_input(day) {
                    Ok(input) => println!("{}", solve(day, args.part, input)),
                    Err(e) => eprintln!("Error: {e}"),
                }
                if day != DAYS {
                    println!();
                }
            }
        }
    }
}
