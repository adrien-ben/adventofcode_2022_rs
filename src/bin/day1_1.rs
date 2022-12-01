use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("inputs/day1").unwrap();
    let reader = BufReader::new(f);

    let mut accum = 0u32;
    let mut max = 0u32;

    reader.lines().map(|l| l.unwrap()).for_each(|cal| {
        if cal == "" {
            if accum > max {
                max = accum;
            }
            accum = 0;
        } else {
            let cal = cal.parse::<u32>().unwrap();
            accum += cal;
        }
    });

    println!("{max}"); // 72602
}
