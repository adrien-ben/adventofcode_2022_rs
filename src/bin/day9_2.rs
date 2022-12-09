use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let moves = parse_moves();
    let mut rope = Rope::default();

    let mut visited_positions = HashSet::new();

    moves.into_iter().for_each(|m| {
        rope.mov(m);
        visited_positions.insert(rope.get_tail_position());
    });

    println!("visited_pos: {}", visited_positions.len()); // 2717
}

#[derive(Clone, Copy, Debug)]
enum Move {
    Left,
    Right,
    Down,
    Up,
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        match s {
            "L" => Self::Left,
            "R" => Self::Right,
            "D" => Self::Down,
            "U" => Self::Up,
            _ => panic!(),
        }
    }
}

fn parse_moves() -> Vec<Move> {
    let f = File::open("inputs/day9").unwrap();
    let reader = BufReader::new(f);

    reader
        .lines()
        .map(|l| l.unwrap())
        .flat_map(|l| {
            let mut split = l.split_ascii_whitespace();
            let dir: Move = split.next().unwrap().into();
            let steps = split.next().unwrap().parse::<u32>().unwrap();

            (0..steps).map(move |_| dir)
        })
        .collect::<Vec<_>>()
}

#[derive(Debug, Default)]
struct Rope {
    knots: [(i32, i32); 10], // 0 is head, 9 is tail
}

impl Rope {
    fn mov(&mut self, mov: Move) {
        match mov {
            Move::Left => self.knots[0].0 -= 1,
            Move::Right => self.knots[0].0 += 1,
            Move::Down => self.knots[0].1 -= 1,
            Move::Up => self.knots[0].1 += 1,
        }

        for i in 1..self.knots.len() {
            let next_move = next_move(self.knots[i], self.knots[i - 1]);

            self.knots[i].0 += next_move.0;
            self.knots[i].1 += next_move.1;
        }
    }

    fn get_tail_position(&self) -> (i32, i32) {
        *self.knots.last().unwrap()
    }
}

fn next_move(pos: (i32, i32), target: (i32, i32)) -> (i32, i32) {
    let h_dist = target.0 - pos.0;
    let v_dist = target.1 - pos.1;

    let mut mov = (0, 0);

    if h_dist.abs() > 1 || v_dist.abs() > 1 {
        // should move
        if h_dist == 0 || v_dist == 0 {
            // same row/column
            if h_dist > 0 {
                // move right
                mov.0 += 1;
            } else if h_dist < 0 {
                // move left
                mov.0 -= 1;
            } else if v_dist > 0 {
                // move up
                mov.1 += 1;
            } else {
                // move down
                mov.1 -= 1;
            }
        } else {
            // diagonal move
            if h_dist > 0 && v_dist > 0 {
                // up/right
                mov.0 += 1;
                mov.1 += 1;
            } else if h_dist > 0 && v_dist < 0 {
                // down/right
                mov.0 += 1;
                mov.1 -= 1;
            } else if h_dist < 0 && v_dist > 0 {
                // up/left
                mov.0 -= 1;
                mov.1 += 1;
            } else {
                // down/left
                mov.0 -= 1;
                mov.1 -= 1;
            }
        }
    }

    mov
}
