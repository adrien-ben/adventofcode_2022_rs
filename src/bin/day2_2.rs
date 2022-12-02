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
            let opponent: Shape = l.chars().nth(0).unwrap().into();
            let outcome: Outcome = l.chars().nth(2).unwrap().into();
            let me = outcome.get_shape(opponent);

            me.get_score(opponent)
        })
        .sum::<u32>();

    println!("{total}"); // 11756
}

#[derive(Clone, Copy)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl From<char> for Outcome {
    fn from(c: char) -> Self {
        match c {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!(),
        }
    }
}

impl Outcome {
    fn get_shape(self, opponent: Shape) -> Shape {
        match (self, opponent) {
            (Self::Lose, Shape::Rock)
            | (Self::Draw, Shape::Scissors)
            | (Self::Win, Shape::Paper) => Shape::Scissors,

            (Self::Lose, Shape::Paper)
            | (Self::Draw, Shape::Rock)
            | (Self::Win, Shape::Scissors) => Shape::Rock,

            (Self::Lose, Shape::Scissors)
            | (Self::Draw, Shape::Paper)
            | (Self::Win, Shape::Rock) => Shape::Paper,
        }
    }
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
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
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
