pub fn part1(input: String) {
    let answer = input.split('\n').fold(0, |acc, line| {
        let (interval1, interval2) = line.split_at(line.find(',').unwrap());
        let interval1 = interval1
            .split('-')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let interval2 = interval2[1..]
            .split('-')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        if interval1[0] <= interval2[0] && interval2[1] <= interval1[1] {
            acc + 1
        } else if interval2[0] <= interval1[0] && interval1[1] <= interval2[1] {
            acc + 1
        } else {
            acc
        }
    });
    println!("{}", answer);
}

pub fn part2(input: String) {
    let answer = input.split('\n').fold(0, |acc, line| {
        let (interval1, interval2) = line.split_at(line.find(',').unwrap());
        let interval1 = interval1
            .split('-')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let interval2 = interval2[1..]
            .split('-')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        if interval1[0] <= interval2[0] && interval2[0] <= interval1[1] {
            acc + 1
        } else if interval2[0] <= interval1[0] && interval1[0] <= interval2[1] {
            acc + 1
        } else {
            acc
        }
    });
    println!("{}", answer);
}
