mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;

pub fn noop(_: String) {
    println!("Not implemented");
}

pub type DayFunction = fn(String);

pub fn get_day(day: u8) -> (DayFunction, DayFunction) {
    match day {
        1 => (day01::part1, day01::part2),
        2 => (day02::part1, day02::part2),
        3 => (day03::part1, day03::part2),
        4 => (day04::part1, day04::part2),
        5 => (day05::part1, day05::part2),
        6 => (day06::part1, day06::part2),
        7 => (day07::part1, day07::part2),
        8 => (day08::part1, day08::part2),
        9 => (day09::part1, day09::part2),
        10 => (day10::part1, day10::part2),
        11 => (day11::part1, day11::part2),
        12 => (day12::part1, day12::part2),
        13 => (day13::part1, day13::part2),
        _ => (noop, noop),
    }
}
