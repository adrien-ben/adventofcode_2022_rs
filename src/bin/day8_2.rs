use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let map = Map::parse();
    let visible_trees_count = map.highest_score();

    println!("{visible_trees_count}"); // 259308
}

struct Map {
    grid: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl Map {
    fn parse() -> Self {
        let f = File::open("inputs/day8").unwrap();
        let reader = BufReader::new(f);

        let grid = reader
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let width = grid[0].len();
        let height = grid.len();

        Self {
            grid,
            width,
            height,
        }
    }

    fn highest_score(&self) -> u32 {
        (1..(self.width - 1))
            .flat_map(|x| (1..(self.height - 1)).map(move |y| (x, y)))
            .map(|(x, y)| self.tree_score(x, y))
            .max()
            .unwrap()
    }

    fn tree_score(&self, x: usize, y: usize) -> u32 {
        let size = self.grid[y][x];

        let mut visible_trees_left = 0u32;
        for i in (0..x).rev() {
            visible_trees_left += 1;
            if size <= self.grid[y][i] {
                break;
            }
        }

        let mut visible_trees_right = 0u32;
        for i in (x + 1)..self.width {
            visible_trees_right += 1;
            if size <= self.grid[y][i] {
                break;
            }
        }

        let mut visible_trees_top = 0u32;
        for i in (0..y).rev() {
            visible_trees_top += 1;
            if size <= self.grid[i][x] {
                break;
            }
        }

        let mut visible_trees_bottom = 0u32;
        for i in (y + 1)..self.height {
            visible_trees_bottom += 1;
            if size <= self.grid[i][x] {
                break;
            }
        }

        visible_trees_left * visible_trees_right * visible_trees_top * visible_trees_bottom
    }
}
