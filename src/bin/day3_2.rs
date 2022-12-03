use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("inputs/day3").unwrap();
    let reader = BufReader::new(f);

    let mut total = 0u32;

    let mut lines = reader.lines();
    loop {
        let elf1 = lines.next();
        if elf1.is_none() {
            break;
        }

        let elf1 = elf1.unwrap().unwrap();
        let elf2 = lines.next().unwrap().unwrap();
        let elf3 = lines.next().unwrap().unwrap();

        let comp1 = elf1.chars().collect::<HashSet<_>>();
        let comp2 = elf2
            .chars()
            .filter(|c| comp1.contains(c))
            .collect::<HashSet<_>>();

        let badge = elf3.chars().find(|c| comp2.contains(c)).unwrap();
        let badge_code = badge as u32;

        if badge_code >= 65 && badge_code < 97 {
            total += badge_code - 38 // map A-Z to 27-52
        } else {
            total += badge_code - 96 // map a-z to 1-26
        }
    }

    println!("{total}"); // 2646
}
