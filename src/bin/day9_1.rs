use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let moves = parse_moves();

    let mut head_pos = (0i32, 0i32);
    let mut tail_pos = head_pos;

    let visited_positions = moves
        .into_iter()
        .filter_map(|m| {
            match m {
                Move::Left => head_pos.0 -= 1,
                Move::Right => head_pos.0 += 1,
                Move::Down => head_pos.1 -= 1,
                Move::Up => head_pos.1 += 1,
            }

            let next_move = next_move(tail_pos, head_pos);

            if next_move != (0, 0) {
                tail_pos.0 += next_move.0;
                tail_pos.1 += next_move.1;
                return Some(tail_pos);
            }

            None
        })
        .collect::<HashSet<_>>();

    println!("visited_pos: {}", visited_positions.len() + 1); // 6522
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
