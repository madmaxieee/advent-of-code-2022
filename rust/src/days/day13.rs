#[derive(Debug, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Number(u64),
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Packet(Packet),
    ListStart,
}

impl Packet {
    fn new(input: &str) -> Self {
        if let Ok(number) = input.parse::<u64>() {
            return Packet::Number(number);
        }

        let mut token_stack: Vec<Token> = vec![];
        let mut current_token = String::new();

        for c in input.chars() {
            match c {
                c if c.is_numeric() => current_token.push(c),
                '[' => {
                    token_stack.push(Token::ListStart);
                }
                ',' => {
                    if !current_token.is_empty() {
                        token_stack.push(Token::new(&current_token));
                        current_token = String::new();
                    }
                }
                ']' => {
                    if !current_token.is_empty() {
                        token_stack.push(Token::new(&current_token));
                        current_token = String::new();
                    }

                    let mut list = vec![];
                    while let Some(token) = token_stack.pop() {
                        match token {
                            Token::ListStart => break,
                            Token::Packet(packet) => list.push(packet),
                        }
                    }
                    list.reverse();
                    token_stack.push(Token::Packet(Packet::List(list)));
                }
                _ => {
                    panic!("Unexpected character: {}", c);
                }
            }
        }

        token_stack.pop().unwrap().unwrap()
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Number(a), Packet::Number(b)) => a.cmp(b),
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
            (Packet::Number(a), Packet::List(b)) => vec![Packet::Number(*a)].cmp(b),
            (Packet::List(a), Packet::Number(b)) => a.cmp(&vec![Packet::Number(*b)]),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Token {
    fn new(input: &str) -> Self {
        Token::Packet(Packet::new(input))
    }

    fn unwrap(self) -> Packet {
        match self {
            Token::Packet(packet) => packet,
            _ => panic!("Token::unwrap() called on non-packet token"),
        }
    }
}

pub fn part1(input: String) {
    let packets: Vec<(Packet, Packet)> = input
        .split("\n\n")
        .map(|line| {
            let mut split = line.split('\n');
            (
                Packet::new(split.next().unwrap()),
                Packet::new(split.next().unwrap()),
            )
        })
        .collect();

    let ans = packets
        .iter()
        .enumerate()
        .filter(|(_, (a, b))| a <= b)
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    println!("{}", ans);
}

pub fn part2(input: String) {
    let packets = {
        let mut packets: Vec<Packet> = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(Packet::new)
            .collect();
        packets.push(Packet::new("[[2]]"));
        packets.push(Packet::new("[[6]]"));
        packets.sort();
        packets
    };

    let ans = packets.iter().enumerate().fold(1, |acc, (i, packet)| {
        if packet == &Packet::new("[[2]]") || packet == &Packet::new("[[6]]") {
            acc * (i + 1)
        } else {
            acc
        }
    });

    println!("{:?}", ans);
}
