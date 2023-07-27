pub fn part1(input: String) {
    let res = input
        .bytes()
        .collect::<Vec<_>>()
        .windows(4)
        .position(|w| {
            let flags: u32 = w.iter().fold(0, |acc, &c| acc | (1 << (c - b'a')));
            flags.count_ones() == 4
        })
        .unwrap();
    println!("{}", res + 4);
}

pub fn part2(input: String) {
    let res = input
        .bytes()
        .collect::<Vec<_>>()
        .windows(14)
        .position(|w| {
            let flags: u32 = w.iter().fold(0, |acc, &c| acc | (1 << (c - b'a')));
            flags.count_ones() == 14
        })
        .unwrap();
    println!("{}", res + 14);
}
