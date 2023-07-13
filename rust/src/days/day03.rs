pub fn part1(input: String) {
    let lines = input.split('\n');
    let mut score: usize = 0;

    lines.for_each(|line| {
        assert!(line.len() % 2 == 0, "Invalid line length");
        let (first_half, second_half) = line.split_at(line.len() / 2);
        let mut flags1 = [false; 52];
        let mut flags2 = [false; 52];
        for (c1, c2) in first_half.chars().zip(second_half.chars()) {
            let index1 = get_index(c1);
            let index2 = get_index(c2);
            flags1[index1] = true;
            flags2[index2] = true;
            if flags1[index2] {
                score += index2 + 1;
                return;
            }
            if flags2[index1] {
                score += index1 + 1;
                return;
            }
        }
    });

    println!("{}", score);
}

pub fn part2(input: String) {
    let lines: Vec<_> = input.split('\n').collect();
    let mut score: usize = 0;

    lines.chunks(3).for_each(|chunk| {
        let flags1: u64 = chunk[0].chars().fold(0, |acc, c| acc | 1 << get_index(c));
        let flags2: u64 = chunk[1].chars().fold(0, |acc, c| acc | 1 << get_index(c));
        let flags = flags1 & flags2;
        for c in chunk[2].chars() {
            let index = get_index(c);
            if flags & (1 << index) != 0 {
                score += index + 1;
                return;
            }
        }
    });

    println!("{}", score);
}

fn get_index(c: char) -> usize {
    let ascii = c as u8;
    if b'A' <= ascii && ascii <= b'Z' {
        (ascii - b'A' + 26) as usize
    } else if b'a' <= ascii && ascii <= b'z' {
        (ascii - b'a') as usize
    } else {
        panic!("Invalid character");
    }
}
