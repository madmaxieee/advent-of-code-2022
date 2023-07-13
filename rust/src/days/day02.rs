pub fn part1(input: String) {
    let lines = input.split('\n');

    let mut score: isize = 0;
    lines.for_each(|line| {
        if line.len() == 0 {
            return;
        }
        let line: Vec<char> = line.chars().collect();
        let shape1: isize = match line[0] {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _ => panic!("Invalid shape"),
        };
        let shape2: isize = match line[2] {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => panic!("Invalid shape"),
        };
        score += shape2 + (shape2 - shape1 + 4) % 3 * 3;
    });

    println!("{}", score);
}

pub fn part2(input: String) {
    let lines = input.split('\n');

    let mut score: isize = 0;
    lines.for_each(|line| {
        if line.len() == 0 {
            return;
        }
        let line: Vec<char> = line.chars().collect();
        let shape: isize = match line[0] {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _ => panic!("Invalid shape"),
        };
        let outcome: isize = match line[2] {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => panic!("Invalid shape"),
        };
        score += (outcome - 1) * 3 + (outcome + shape) % 3 + 1;
    });

    println!("{}", score);
}
