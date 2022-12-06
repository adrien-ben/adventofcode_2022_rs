use std::fs::File;

const BUFFER_SIZE: usize = 4;

fn main() {
    let f = File::open("inputs/day6").unwrap();

    let data = std::io::read_to_string(f).unwrap();

    let mut buffer = [' '; BUFFER_SIZE];
    let mut cursor = 0usize;

    for (i, c) in data.chars().enumerate() {
        buffer[cursor] = c;

        if i >= BUFFER_SIZE && all_different(&buffer) {
            println!("{}", i + 1); // 1640
            break;
        }

        cursor = (cursor + 1) % BUFFER_SIZE;
    }
}

fn all_different(buffer: &[char; BUFFER_SIZE]) -> bool {
    buffer[0] != buffer[1]
        && buffer[0] != buffer[2]
        && buffer[0] != buffer[3]
        && buffer[1] != buffer[2]
        && buffer[1] != buffer[3]
        && buffer[2] != buffer[3]
}
