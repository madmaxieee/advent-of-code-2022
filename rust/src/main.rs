mod days;

use days::get_day;
use std::env;
use std::fs;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2, "Usage: cargo run <day>");

    let day: u8 = match args[1].parse() {
        Ok(day) => day,
        Err(_) => panic!("Day must be a number"),
    };
    assert!((1..=25).contains(&day), "Day must be between 1 and 25");

    let file_name = format!("inputs/day{:02}.txt", day);
    let input = match fs::read_to_string(file_name.clone()) {
        Ok(input) => input,
        Err(_) => panic!("Could not read file {}", file_name.clone()),
    };

    let (part1, part2) = get_day(day);

    println!("Part 1:");
    let start = Instant::now();
    part1(input.clone());
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    println!();

    println!("Part 2:");
    let start = Instant::now();
    part2(input.clone());
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    Ok(())
}
