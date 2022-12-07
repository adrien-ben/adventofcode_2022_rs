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

    let occupied_space = dirs.get(&vec!["/".to_string()]).unwrap();
    let free_space = 70_000_000 - occupied_space;
    let space_to_free = 30_000_000 - free_space;

    let mut sizes = dirs.values().copied().collect::<Vec<_>>();
    sizes.sort();
    let size_of_folder_to_delete = sizes.into_iter().find(|s| *s >= space_to_free).unwrap();

    println!("{size_of_folder_to_delete}"); // 942298
}
