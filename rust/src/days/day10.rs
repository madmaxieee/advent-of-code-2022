#[derive(Clone, Debug)]
enum Instruction {
    NoOp,
    AddX(i32),
}

impl Instruction {
    fn duration(&self) -> i32 {
        match self {
            Instruction::NoOp => 1,
            Instruction::AddX(_) => 2,
        }
    }
}

pub fn part1(input: String) {
    let instructions = process_input(input);
    let mut instructions = instructions.iter();
    let mut cycle = 0;
    let mut x_reg = 1;

    let mut signal_strength = 0;

    let mut current_instr = instructions.next().unwrap();
    let mut instr_start = 0;

    while cycle <= 220 {
        cycle += 1;

        if (cycle - 20) % 40 == 0 {
            signal_strength += x_reg * cycle;
        }
        if cycle - instr_start >= current_instr.clone().duration() {
            match current_instr {
                Instruction::NoOp => {}
                Instruction::AddX(x) => {
                    x_reg += x;
                }
            }
            instr_start = cycle;
            current_instr = instructions.next().unwrap();
        }
    }

    println!("{}", signal_strength);
}

pub fn part2(input: String) {
    let instructions = process_input(input);
    let mut instructions = instructions.iter();

    let mut cycle = 0;
    let mut x_reg = 1;

    let mut current_instr = instructions.next().unwrap();
    let mut instr_start = 0;

    let mut screen = vec![false; 240];

    while cycle < 240 {
        if (x_reg - 1..=x_reg + 1).contains(&(cycle % 40_i32)) {
            screen[cycle as usize] = true;
        }

        cycle += 1;
        if cycle - instr_start >= current_instr.clone().duration() {
            match current_instr {
                Instruction::NoOp => {}
                Instruction::AddX(x) => x_reg += x,
            }
            instr_start = cycle;
            current_instr = instructions.next().unwrap_or(&Instruction::NoOp);
        }
    }

    println!(
        "{}",
        screen
            .iter()
            .map(|b| if *b { '#' } else { '.' })
            .collect::<Vec<char>>()
            .chunks(40)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );
}

fn process_input(input: String) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            match parts.next().unwrap() {
                "noop" => Instruction::NoOp,
                "addx" => Instruction::AddX(parts.next().unwrap().parse().unwrap()),
                _ => panic!("Unknown instruction"),
            }
        })
        .collect::<Vec<Instruction>>()
}
