mod days;

use days::get_day;
use std::env;
use std::fs;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2, "Usage: cargo run <day>");

    let day = args[1].parse::<u8>()?;
    assert!(day >= 1 && day <= 25, "Day must be between 1 and 25");

    let input = fs::read_to_string(format!("inputs/day{:02}.txt", day))?;

    let (part1, part2) = get_day(day);

    println!("Part 1:");
    let start = Instant::now();
    part1(input.clone());
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    println!("");

    println!("Part 2:");
    let start = Instant::now();
    part2(input.clone());
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    Ok(())
}
