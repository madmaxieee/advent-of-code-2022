mod day01;

pub fn noop(_: String) {}

pub type DayFunction = fn(String);

pub fn get_day(day: u8) -> (DayFunction, DayFunction) {
    match day {
        1 => (day01::part1, day01::part2),
        _ => (noop, noop),
    }
}
