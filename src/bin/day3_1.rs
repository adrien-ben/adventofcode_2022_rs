use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("inputs/day3").unwrap();
    let reader = BufReader::new(f);

    let total = reader
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let length = l.len();
            let half = length / 2;

            let first_compartment = l.chars().take(half).collect::<HashSet<_>>();
            let error = l
                .chars()
                .skip(half)
                .find(|c| first_compartment.contains(c))
                .unwrap();
            let error_code = error as u32;

            if (64..97).contains(&error_code) {
                error_code - 38 // map A-Z to 27-52
            } else {
                error_code - 96 // map a-z to 1-26
            }
        })
        .sum::<u32>();

    println!("{total}"); // 7446
}
