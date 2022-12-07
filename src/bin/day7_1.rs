use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("inputs/day7").unwrap();
    let reader = BufReader::new(f);

    let mut dirs = HashMap::new(); // maps path to size
    dirs.insert(vec!["/".to_string()], 0u32);

    let mut path = vec!["/".to_string()]; // the path of the currently visited dir

    reader.lines().map(|l| l.unwrap()).for_each(|l| {
        let mut split = l.split_ascii_whitespace();

        let first = split.next().unwrap();

        // command
        if first == "$" {
            let command = split.next().unwrap();
            if command == "cd" {
                let target = split.next().unwrap();
                if target == "/" {
                    path = vec!["/".to_string()];
                } else if target == ".." {
                    path.pop();
                } else {
                    path.push(target.to_string());
                }
            }
        }

        // file
        if let Ok(size) = first.parse::<u32>() {
            let mut key = vec![];

            // add the size of the file to all dirs composing the current path
            path.iter().cloned().for_each(|d| {
                key.push(d);

                dirs.entry(key.clone())
                    .and_modify(|s| *s += size)
                    .or_insert(size);
            });
        }
    });

    let total_size_of_small_dirs = dirs.values().filter(|s| **s < 100000).sum::<u32>();

    println!("{total_size_of_small_dirs}"); // 1443806
}
