use std::fs::File;

const BUFFER_SIZE: usize = 14;

fn main() {
    let f = File::open("inputs/day6").unwrap();

    let data = std::io::read_to_string(f).unwrap();

    let mut buffer = [' '; BUFFER_SIZE];
    let mut cursor = 0usize;

    for (i, c) in data.chars().enumerate() {
        buffer[cursor] = c;

        if i >= BUFFER_SIZE && all_different(&buffer) {
            println!("{}", i + 1); // 3613
            break;
        }

        cursor = (cursor + 1) % BUFFER_SIZE;
    }
}

fn all_different(buffer: &[char]) -> bool {
    for (i, c1) in buffer.iter().enumerate() {
        for c2 in buffer.iter().skip(i + 1) {
            if c1 == c2 {
                return false;
            }
        }
    }

    true
}

fn _all_different_with_iter(buffer: &[char]) -> bool {
    buffer
        .iter()
        .enumerate()
        .flat_map(|(i, c1)| buffer.iter().skip(i + 1).map(|c2| (*c1, *c2)))
        .find(|(c1, c2)| c1 == c2)
        .is_none()
}
