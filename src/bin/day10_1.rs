use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("inputs/day10").unwrap();
    let reader = BufReader::new(f);

    let sum_of_strength = reader
        .lines()
        .map(|l| Command::parse(&l.unwrap()))
        .scan((1, 1), |(cycle, register), cmd| {
            let (cycles, value) = match cmd {
                Command::Noop => (1, 0),
                Command::Addx(v) => (2, v),
            };

            let strength = (0..cycles)
                .map(|i| *cycle + i)
                .filter(|c| [20, 60, 100, 140, 180, 220].contains(c))
                .map(|c| *register * c)
                .sum::<i32>();

            *cycle += cycles;
            *register += value;

            Some(strength)
        })
        .sum::<i32>();

    println!("Sum of strength: {sum_of_strength}"); // 11960
}

#[derive(Debug, Clone, Copy)]
enum Command {
    Noop,
    Addx(i32),
}

impl Command {
    fn parse(line: &str) -> Self {
        let mut split = line.split_ascii_whitespace();
        let cmd = split.next().unwrap();

        if cmd == "noop" {
            Command::Noop
        } else {
            let v = split.next().unwrap().parse::<i32>().unwrap();
            Command::Addx(v)
        }
    }
}
