use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let map = Map::parse();
    let visible_trees_count = map.visible_trees_count();

    println!("{visible_trees_count}"); // 1798
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

    fn visible_trees_count(&self) -> u32 {
        let edge_trees_count = 2 * self.width + 2 * (self.height - 2);

        let mut visible_trees = 0u32;
        for x in 1..(self.width - 1) {
            for y in 1..(self.height - 1) {
                if self.is_tree_visible(x, y) {
                    visible_trees += 1;
                }
            }
        }

        edge_trees_count as u32 + visible_trees
    }

    fn is_tree_visible(&self, x: usize, y: usize) -> bool {
        let mut occluding_trees = 0u32;

        let size = self.grid[y][x];

        for i in 0..x {
            if size <= self.grid[y][i] {
                occluding_trees += 1;
                break;
            }
        }

        for i in (x + 1)..self.width {
            if size <= self.grid[y][i] {
                occluding_trees += 1;
                break;
            }
        }

        for i in 0..y {
            if size <= self.grid[i][x] {
                occluding_trees += 1;
                break;
            }
        }

        for i in (y + 1)..self.height {
            if size <= self.grid[i][x] {
                occluding_trees += 1;
                break;
            }
        }

        occluding_trees < 4
    }
}
