use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("inputs/day2").unwrap();
    let reader = BufReader::new(f);

    let total = reader
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let opponent: Shape = l.chars().next().unwrap().into();
            let me: Shape = l.chars().nth(2).unwrap().into();

            me.get_score(opponent)
        })
        .sum::<u32>();

    println!("{total}"); // 12645
}

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Shape {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!(),
        }
    }
}

impl Shape {
    fn get_score(self, opponent: Shape) -> u32 {
        let shape_score = self.value();

        let round_score = match (self, opponent) {
            (Self::Rock, Self::Rock)
            | (Self::Paper, Self::Paper)
            | (Self::Scissors, Self::Scissors) => 3,

            (Self::Rock, Self::Paper)
            | (Self::Paper, Self::Scissors)
            | (Self::Scissors, Self::Rock) => 0,

            (Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper) => 6,
        };

        shape_score + round_score
    }

    fn value(self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}
