use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("inputs/day1").unwrap();
    let reader = BufReader::new(f);

    let mut accum = 0u32;
    let mut totals = vec![];

    reader.lines().map(|l| l.unwrap()).for_each(|cal| {
        if cal == "" {
            totals.push(accum);
            accum = 0;
        } else {
            let cal = cal.parse::<u32>().unwrap();
            accum += cal;
        }
    });

    totals.sort();

    let total_of_3_highest = totals.iter().rev().take(3).sum::<u32>();

    println!("{total_of_3_highest}"); // 207410
}
