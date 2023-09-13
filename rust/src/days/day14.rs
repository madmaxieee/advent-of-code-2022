use std::collections::HashSet;

pub fn part1(input: String) {
    let mut rocks = parse_rocks(input);

    let bottom_most = rocks.iter().max_by_key(|(_, y)| *y).unwrap().1;
    let mut count = 0;

    'simulation: loop {
        let mut sand = (500, 0);
        loop {
            if sand.1 >= bottom_most {
                break 'simulation;
            }
            if !rocks.contains(&(sand.0, sand.1 + 1)) {
                sand.1 += 1;
            } else if !rocks.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand.0 -= 1;
                sand.1 += 1;
            } else if !rocks.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand.0 += 1;
                sand.1 += 1;
            } else {
                rocks.insert(sand);
                count += 1;
                break;
            }
        }
    }

    println!("{}", count);
}

pub fn part2(input: String) {
    let mut rocks = parse_rocks(input);

    let floor = rocks.iter().max_by_key(|(_, y)| *y).unwrap().1 + 2;
    let mut count = 0;

    'simulation: loop {
        let mut sand = (500, 0);
        loop {
            if !rocks.contains(&(sand.0, sand.1 + 1)) && sand.1 + 1 < floor {
                sand.1 += 1;
            } else if !rocks.contains(&(sand.0 - 1, sand.1 + 1)) && sand.1 + 1 < floor {
                sand.0 -= 1;
                sand.1 += 1;
            } else if !rocks.contains(&(sand.0 + 1, sand.1 + 1)) && sand.1 + 1 < floor {
                sand.0 += 1;
                sand.1 += 1;
            } else {
                rocks.insert(sand);
                count += 1;
                if sand == (500, 0) {
                    break 'simulation;
                } else {
                    break;
                }
            }
        }
    }

    println!("{}", count);
}

fn parse_rocks(input: String) -> HashSet<(i32, i32)> {
    let mut rocks = HashSet::new();

    input.lines().for_each(|line| {
        let mut points = line.split(" -> ").map(|point| {
            let point = point
                .split(',')
                .map(|coord| coord.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (point[0], point[1])
        });

        let mut current = points.next().unwrap();

        for point in points {
            while current.0 != point.0 {
                rocks.insert(current);
                if current.0 < point.0 {
                    current.0 += 1;
                } else {
                    current.0 -= 1;
                }
            }
            while current.1 != point.1 {
                rocks.insert(current);
                if current.1 < point.1 {
                    current.1 += 1;
                } else {
                    current.1 -= 1;
                }
            }
        }

        rocks.insert(current);
    });

    rocks
}
