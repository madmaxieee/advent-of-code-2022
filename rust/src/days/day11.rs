use std::collections::VecDeque;

#[derive(Debug)]
struct Monkey {
    #[allow(dead_code)]
    id: usize,
    items: VecDeque<i64>,
    operation: Operation,
    test: Test,
    inspections: usize,
}

impl Monkey {
    fn new(id: usize, items: VecDeque<i64>, operation: Operation, test: Test) -> Self {
        Self {
            id,
            items,
            operation,
            test,
            inspections: 0,
        }
    }

    fn inspect1(&mut self) -> Result<Throw, ()> {
        let item = self.items.pop_front().ok_or(())?;
        let worry_level = match self.operation {
            Operation::Add(n) => item + n,
            Operation::Multiply(n) => item * n,
            Operation::Square => item * item,
        };

        let worry_level = worry_level / 3;
        let next_monkey = if worry_level % self.test.divisor == 0 {
            self.test.true_monkey
        } else {
            self.test.false_monkey
        };

        self.inspections += 1;
        Ok(Throw(next_monkey, worry_level))
    }

    fn inspect2(&mut self, lcm: i64) -> Result<Throw, ()> {
        let item = self.items.pop_front().ok_or(())?;
        let worry_level = match self.operation {
            Operation::Add(n) => item + n,
            Operation::Multiply(n) => item * n,
            Operation::Square => item * item,
        };

        let worry_level = worry_level % lcm;
        let next_monkey = if worry_level % self.test.divisor == 0 {
            self.test.true_monkey
        } else {
            self.test.false_monkey
        };

        self.inspections += 1;
        Ok(Throw(next_monkey, worry_level))
    }
}

#[derive(Debug)]
enum Operation {
    Add(i64),
    Multiply(i64),
    Square,
}

// divisor, true, false
#[derive(Debug)]
struct Test {
    divisor: i64,
    true_monkey: usize,
    false_monkey: usize,
}

impl Test {
    fn new(divisor: i64, true_monkey: usize, false_monkey: usize) -> Self {
        Self {
            divisor,
            true_monkey,
            false_monkey,
        }
    }
}

// monkey id, worry level
#[derive(Debug)]
struct Throw(usize, i64);

impl Operation {
    fn new(operation: (&str, &str)) -> Self {
        match operation {
            ("*", "old") => Self::Square,
            ("+", operand) => Self::Add(operand.parse::<i64>().unwrap()),
            ("*", operand) => Self::Multiply(operand.parse::<i64>().unwrap()),
            _ => panic!("Invalid operation"),
        }
    }
}

pub fn part1(input: String) {
    let mut monkeys = process_input(input);

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Ok(Throw(id, worry_level)) = monkeys[i].inspect1() {
                monkeys[id].items.push_back(worry_level);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));

    let monkey_business = monkeys[0].inspections * monkeys[1].inspections;
    println!("{:?}", monkey_business);
}

pub fn part2(input: String) {
    let mut monkeys = process_input(input);
    let lcm = monkeys.iter().map(|m| m.test.divisor).product::<i64>();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while let Ok(Throw(id, worry_level)) = monkeys[i].inspect2(lcm) {
                // println!("{:?}", worry_level);
                monkeys[id].items.push_back(worry_level);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));

    let monkey_business = monkeys[0].inspections * monkeys[1].inspections;
    println!("{:?}", monkey_business);
}

fn process_input(input: String) -> Vec<Monkey> {
    let monkey_notes = input.split("\n\n");

    let monkeys: Vec<Monkey> = monkey_notes
        .map(|note| {
            let note = note.lines().collect::<Vec<&str>>();

            let id = note[0]
                .split(' ')
                .nth(1)
                .unwrap()
                .trim_end_matches(':')
                .parse::<usize>()
                .unwrap();

            let items = note[1]
                .split(": ")
                .nth(1)
                .unwrap()
                .split(", ")
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<VecDeque<i64>>();

            let operation = note[2]
                .split(": new = old ")
                .nth(1)
                .unwrap()
                .split(' ')
                .collect::<Vec<&str>>();
            let operation = Operation::new((operation[0], operation[1]));

            let divisor = note[3]
                .split(": divisible by ")
                .nth(1)
                .unwrap()
                .parse::<i64>()
                .unwrap();
            let true_monkey = note[4]
                .split(": throw to monkey ")
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let false_monkey = note[5]
                .split(": throw to monkey ")
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let test = Test::new(divisor, true_monkey, false_monkey);

            Monkey::new(id, items, operation, test)
        })
        .collect();

    monkeys
}
