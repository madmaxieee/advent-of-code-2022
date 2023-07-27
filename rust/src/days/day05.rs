pub fn part1(input: String) {
    let mut parts = input.split("\n\n");
    let mut initial_condition = parts.next().unwrap().lines().rev();
    let operations = parts.next().unwrap();

    let num_stacks = (initial_condition.next().unwrap().len() + 1) / 4;
    let mut stacks = vec![Vec::<char>::new(); num_stacks];

    initial_condition.for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(i, c)| {
                if c != ' ' {
                    stacks[i].push(c);
                }
            });
    });

    operations.lines().for_each(|operation| {
        let mut parts = operation
            .split(' ')
            .skip(1)
            .step_by(2)
            .map(|n| n.parse::<usize>().unwrap());

        let (n, src, dest) = (
            parts.next().unwrap(),
            parts.next().unwrap() - 1,
            parts.next().unwrap() - 1,
        );

        for _ in 0..n {
            let tmp = stacks[src].pop().unwrap();
            stacks[dest].push(tmp);
        }
    });

    stacks
        .iter()
        .for_each(|stack| print!("{}", stack.last().unwrap()));
    println!();
}

pub fn part2(input: String) {
    let mut parts = input.split("\n\n");
    let mut initial_condition = parts.next().unwrap().lines().rev();
    let operations = parts.next().unwrap();

    let num_stacks = (initial_condition.next().unwrap().len() + 1) / 4;
    let mut stacks = vec![Vec::<char>::new(); num_stacks];

    initial_condition.for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(i, c)| {
                if c != ' ' {
                    stacks[i].push(c);
                }
            });
    });

    operations.lines().for_each(|operation| {
        let mut parts = operation
            .split(' ')
            .skip(1)
            .step_by(2)
            .map(|n| n.parse::<usize>().unwrap());

        let (n, src, dest) = (
            parts.next().unwrap(),
            parts.next().unwrap() - 1,
            parts.next().unwrap() - 1,
        );

        let mut tmp_stack = Vec::<char>::new();
        for _ in 0..n {
            tmp_stack.push(stacks[src].pop().unwrap());
        }
        tmp_stack.iter().rev().for_each(|c| stacks[dest].push(*c));
    });

    stacks
        .iter()
        .for_each(|stack| print!("{}", stack.last().unwrap()));
    println!();
}
