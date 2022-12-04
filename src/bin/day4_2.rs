use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("inputs/day4").unwrap();
    let reader = BufReader::new(f);

    let overlapping_pairs = reader
        .lines()
        .filter(|l| {
            let l = l.as_ref().unwrap();

            let (left_pair, right_pair) = l.split_once(',').unwrap();
            let (left_start, left_end) = parse_pair(left_pair);
            let (right_start, right_end) = parse_pair(right_pair);

            left_end - right_start >= 0 && right_end - left_start >= 0
        })
        .count();

    println!("{overlapping_pairs}"); // 905
}

fn parse_pair(pair: &str) -> (i32, i32) {
    let (left, right) = pair.split_once('-').unwrap();

    (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap())
}
