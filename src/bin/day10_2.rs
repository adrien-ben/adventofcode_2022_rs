use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("inputs/day10").unwrap();
    let reader = BufReader::new(f);

    let (_cpu, crt) = reader.lines().map(|l| Command::parse(&l.unwrap())).fold(
        (Cpu::default(), Crt::default()),
        |(mut cpu, mut crt), cmd| {
            let (cycles, value) = match cmd {
                Command::Noop => (1, 0),
                Command::Addx(v) => (2, v),
            };

            (0..cycles).for_each(|_| {
                crt.draw(&cpu);
                cpu.cycle += 1;
            });

            cpu.register += value;

            (cpu, crt)
        },
    );

    println!("CRT: {crt}"); // EJCFPGLH
}

#[derive(Debug, Clone, Copy)]
enum Command {
    Noop,
    Addx(i32),
}

impl Command {
    fn parse(line: &str) -> Self {
        let mut split = line.split_ascii_whitespace();
        let cmd = split.next().unwrap();

        if cmd == "noop" {
            Command::Noop
        } else {
            let v = split.next().unwrap().parse::<i32>().unwrap();
            Command::Addx(v)
        }
    }
}

struct Cpu {
    cycle: usize,
    register: i32,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            cycle: 0,
            register: 1,
        }
    }
}

struct Crt {
    buffer: [char; 40 * 6],
}

impl Crt {
    fn draw(&mut self, cpu: &Cpu) {
        let line = cpu.cycle / 40;
        let current_pixel = cpu.cycle;
        let current_pixel_position_in_line = current_pixel - (line * 40);

        if ((cpu.register - 1)..=(cpu.register + 1))
            .contains(&(current_pixel_position_in_line as i32))
        {
            self.buffer[current_pixel] = '#';
        }
    }
}

impl Default for Crt {
    fn default() -> Self {
        Self {
            buffer: ['.'; 40 * 6],
        }
    }
}

impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, c) in self.buffer.iter().enumerate() {
            if i % 40 == 0 {
                writeln!(f)?;
            }
            write!(f, "{c}")?;
        }

        Ok(())
    }
}
