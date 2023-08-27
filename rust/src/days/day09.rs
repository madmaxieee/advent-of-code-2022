use std::{collections::HashSet, fmt, ops};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

pub fn part1(input: String) {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited_points: HashSet<(i32, i32)> = HashSet::new();
    visited_points.insert(tail);

    input.lines().for_each(|line| {
        let mut line = line.split_whitespace();

        match line.next() {
            Some("U") => {
                for _ in 0..line.next().unwrap().parse::<usize>().unwrap() {
                    head.1 += 1;
                    if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                        tail = (head.0, head.1 - 1);
                        visited_points.insert(tail);
                    }
                }
            }
            Some("D") => {
                for _ in 0..line.next().unwrap().parse::<usize>().unwrap() {
                    head.1 -= 1;
                    if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                        tail = (head.0, head.1 + 1);
                        visited_points.insert(tail);
                    }
                }
            }
            Some("L") => {
                for _ in 0..line.next().unwrap().parse::<usize>().unwrap() {
                    head.0 -= 1;
                    if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                        tail = (head.0 + 1, head.1);
                        visited_points.insert(tail);
                    }
                }
            }
            Some("R") => {
                for _ in 0..line.next().unwrap().parse::<usize>().unwrap() {
                    head.0 += 1;
                    if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                        tail = (head.0 - 1, head.1);
                        visited_points.insert(tail);
                    }
                }
            }
            _ => panic!("Invalid input"),
        };
    });

    let ans = visited_points.len();
    println!("{}", ans);
}

pub fn part2(input: String) {
    let mut knots: Vec<Point> = vec![Point(0, 0); 10];
    let mut visited_points: HashSet<Point> = HashSet::new();
    visited_points.insert(Point(0, 0));

    input.lines().for_each(|line| {
        println!("{}", line);

        let mut line = line.split_whitespace();

        let direction = match line.next() {
            Some("U") => Direction::Up,
            Some("D") => Direction::Down,
            Some("L") => Direction::Left,
            Some("R") => Direction::Right,
            _ => panic!("Invalid input"),
        };

        for _ in 0..line.next().unwrap().parse::<usize>().unwrap() {
            match direction {
                Direction::Up => knots[0].1 += 1,
                Direction::Down => knots[0].1 -= 1,
                Direction::Left => knots[0].0 -= 1,
                Direction::Right => knots[0].0 += 1,
            };
            for i in 0..9 {
                let distance = (
                    (knots[i].0 - knots[i + 1].0).abs(),
                    (knots[i].1 - knots[i + 1].1).abs(),
                );

                if distance.0 > 1 && distance.1 == 0 {
                    knots[i + 1].0 += if knots[i].0 > knots[i + 1].0 { 1 } else { -1 };
                } else if distance.1 > 1 && distance.0 == 0 {
                    knots[i + 1].1 += if knots[i].1 > knots[i + 1].1 { 1 } else { -1 };
                } else if distance.0 > 1 || distance.1 > 1 {
                    knots[i + 1].0 += if knots[i].0 > knots[i + 1].0 { 1 } else { -1 };
                    knots[i + 1].1 += if knots[i].1 > knots[i + 1].1 { 1 } else { -1 };
                } else {
                    break;
                }
            }
            visited_points.insert(knots[9]);
        }
    });

    let ans = visited_points.len();
    println!("{}", ans);
}
