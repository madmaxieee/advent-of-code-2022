mod day01;
mod day02;

pub fn noop(_: String) {
    println!("Not implemented");
}

pub type DayFunction = fn(String);

pub fn get_day(day: u8) -> (DayFunction, DayFunction) {
    match day {
        1 => (day01::part1, day01::part2),
        2 => (day02::part1, day02::part2),
        _ => (noop, noop),
    }
}
