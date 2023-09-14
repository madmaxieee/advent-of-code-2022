use regex::Regex;
use std::{
    cmp::{max, min},
    collections::HashSet,
};

#[derive(Debug)]
struct Sensor {
    position: (i32, i32),
    closest: (i32, i32),
}

impl Sensor {
    fn new(position: (i32, i32), closest: (i32, i32)) -> Self {
        Self { position, closest }
    }
}

pub fn part1(_input: String) {
    let sensors = parse_input(_input);
    let target_y = 2000000;

    let mut blocked_intervals: Vec<(i32, i32)> = Vec::new();
    for sensor in sensors.iter() {
        let distance = (sensor.position.0 - sensor.closest.0).abs()
            + (sensor.position.1 - sensor.closest.1).abs();
        let range = distance - (sensor.position.1 - target_y).abs();
        if range >= 0 {
            blocked_intervals.push((sensor.position.0 - range, sensor.position.0 + range));
        }
    }
    blocked_intervals.sort();

    let mut merged_intervals: Vec<(i32, i32)> = Vec::new();
    let mut ans = 0;
    let mut current_interval = blocked_intervals[0];
    for (start, end) in blocked_intervals.iter().skip(1) {
        if *start <= current_interval.1 {
            current_interval.1 = max(current_interval.1, *end);
        } else {
            ans += current_interval.1 - current_interval.0 + 1;
            merged_intervals.push(current_interval);
            current_interval = (*start, *end);
        }
    }
    merged_intervals.push(current_interval);
    ans += current_interval.1 - current_interval.0 + 1;

    let mut removed: HashSet<i32> = HashSet::new();
    sensors.iter().for_each(|s| {
        if removed.contains(&s.closest.0) {
            return;
        }
        if merged_intervals
            .iter()
            .any(|(start, end)| s.closest.1 == target_y && (start..=end).contains(&&s.closest.0))
        {
            removed.insert(s.closest.0);
            ans -= 1;
        }
    });

    println!("{}", ans);
}

pub fn part2(_input: String) {
    let sensors = parse_input(_input);

    for y in 0..=4_000_000 {
        let blocked_intervals = search_row(y, &sensors);
        if !blocked_intervals
            .iter()
            .any(|(start, end)| *start <= 0 && *end >= 4_000_000)
        {
            for intervals in blocked_intervals.windows(2) {
                if intervals[0].1 < intervals[1].0 {
                    println!("{}", (intervals[0].1 + 1) as i64 * 4_000_000 + y as i64);
                    return;
                }
            }
        }
    }

    unreachable!();
}

fn parse_input(_input: String) -> Vec<Sensor> {
    let re = Regex::new(
        r"Sensor at x=([0-9]+), y=([0-9]+): closest beacon is at x=([0-9]+), y=([0-9]+)",
    )
    .unwrap();

    re.captures_iter(_input.as_str())
        .map(|c| c.extract())
        .map(|(_, [x, y, bx, by])| {
            Sensor::new(
                (x.parse().unwrap(), y.parse().unwrap()),
                (bx.parse().unwrap(), by.parse().unwrap()),
            )
        })
        .collect()
}

fn search_row(y: i32, sensors: &[Sensor]) -> Vec<(i32, i32)> {
    let mut blocked_intervals: Vec<(i32, i32)> = Vec::new();
    for sensor in sensors.iter() {
        let distance = (sensor.position.0 - sensor.closest.0).abs()
            + (sensor.position.1 - sensor.closest.1).abs();
        let range = distance - (sensor.position.1 - y).abs();
        if range >= 0 {
            blocked_intervals.push((
                max(0, sensor.position.0 - range),
                min(4_000_000, sensor.position.0 + range),
            ));
        }
    }
    blocked_intervals.sort();

    let mut merged_intervals: Vec<(i32, i32)> = Vec::new();
    let mut current_interval = blocked_intervals[0];
    for (start, end) in blocked_intervals.iter().skip(1) {
        if *start <= current_interval.1 {
            current_interval.1 = max(current_interval.1, *end);
        } else {
            merged_intervals.push(current_interval);
            current_interval = (*start, *end);
        }
    }
    merged_intervals.push(current_interval);

    merged_intervals
}
