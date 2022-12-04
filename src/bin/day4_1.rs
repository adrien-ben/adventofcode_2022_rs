use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("inputs/day4").unwrap();
    let reader = BufReader::new(f);

    let contained_pairs = reader
        .lines()
        .filter(|l| {
            let l = l.as_ref().unwrap();

            let (left_pair, right_pair) = l.split_once(',').unwrap();
            let (left_start, left_end) = parse_pair(left_pair);
            let (right_start, right_end) = parse_pair(right_pair);

            (left_start <= right_start && left_end >= right_end)
                || (right_start <= left_start && right_end >= left_end)
        })
        .count();

    println!("{contained_pairs}"); // 576
}

fn parse_pair(pair: &str) -> (u32, u32) {
    let (left, right) = pair.split_once('-').unwrap();

    (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap())
}
