use std::{
    fs::File,
    io::{BufRead, BufReader, Seek, SeekFrom},
};

fn main() {
    let f = File::open("inputs/day5").unwrap();

    let mut crates = Crates::parse(&f);
    let moves = parse_moves(&f);

    moves.into_iter().for_each(|m| crates.move_crates(m));

    crates.print_message(); // VQZNJMWTR
}

struct Crates {
    stacks: Vec<Vec<char>>,
}

impl Crates {
    fn parse(file: &File) -> Self {
        let reader = BufReader::new(file);

        let mut stacks = Vec::new();

        reader
            .lines()
            .map(|l| l.unwrap())
            .take_while(|l| !l.starts_with(" 1"))
            .for_each(|l| {
                let mut iter = 0usize;
                let mut chars = l.chars();
                loop {
                    if stacks.len() <= iter {
                        stacks.push(Vec::new());
                    }

                    chars.next(); // [ o ' '

                    let crate_code = chars.next().unwrap();
                    if crate_code != ' ' {
                        stacks[iter].insert(0, crate_code);
                    }

                    chars.next(); // ] or ' '

                    if chars.next().is_none() {
                        // end of line
                        break;
                    }

                    iter += 1;
                }
            });

        Self { stacks }
    }

    fn move_crates(&mut self, movement: Movement) {
        for _ in 0..movement.count {
            let to_move = self.stacks[movement.source].pop().unwrap();
            self.stacks[movement.target].push(to_move);
        }
    }

    fn print_message(&self) {
        self.stacks
            .iter()
            .for_each(|s| print!("{}", s.last().unwrap()));
    }
}

#[derive(Clone, Copy, Debug)]
struct Movement {
    source: usize,
    target: usize,
    count: usize,
}

impl Movement {
    fn parse(text: &str) -> Self {
        let mut split = text.split_ascii_whitespace();
        let count = split.nth(1).unwrap().parse::<usize>().unwrap();
        let source = split.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let target = split.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        Self {
            source,
            target,
            count,
        }
    }
}

fn parse_moves(file: &File) -> Vec<Movement> {
    let mut reader = BufReader::new(file);
    reader.seek(SeekFrom::Start(0)).unwrap();

    reader
        .lines()
        .map(|l| l.unwrap())
        .skip_while(|l| !l.starts_with("move"))
        .map(|l| Movement::parse(&l))
        .collect::<Vec<_>>()
}
