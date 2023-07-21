use std::cmp;

pub fn part1(input: String) {
    let mut max_sum = 0;
    let mut current_sum = 0;

    for line in input.lines() {
        if line.is_empty() {
            max_sum = cmp::max(max_sum, current_sum);
            current_sum = 0;
        } else {
            let calories = line.parse::<usize>().unwrap();
            current_sum += calories;
        }
    }

    println!("{}", max_sum);
}

#[allow(dead_code)]
fn part1_2(input: String) {
    println!(
        "{}",
        input
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .map(|calories| calories.parse::<usize>().unwrap())
                    .sum::<usize>()
            })
            .max()
            .unwrap()
    );
}

pub fn part2(input: String) {
    let lines = input.split('\n');

    let mut top3 = [0; 3];
    let mut current_sum = 0;

    for line in lines {
        if line.is_empty() {
            if current_sum > top3[0] {
                top3[0] = current_sum;
                top3.sort();
            }
            current_sum = 0;
        } else {
            let calories = line.parse::<usize>().unwrap();
            current_sum += calories;
        }
    }

    println!("{}", top3.iter().sum::<usize>());
}
