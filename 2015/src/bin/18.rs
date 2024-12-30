#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
const DIRS8: [(isize, isize); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

fn neighbors_on(grid: &[Vec<bool>], idx: (usize, usize)) -> usize {
    let mut out = 0;
    for d in DIRS8 {
        let x = idx.0 as isize + d.0;
        let y = idx.1 as isize + d.1;
        if x >= 0
            && (x as usize) < grid.len()
            && y >= 0
            && (y as usize) < grid[0].len()
            && grid[x as usize][y as usize]
        {
            out += 1;
        }
    }

    out
}

fn part1(input: &str) -> impl Debug {
    let mut grid = grid_with(input, |c| c == '#');
    for _ in 0..100 {
        grid = (0..grid.len())
            .map(|i| {
                (0..grid[i].len())
                    .map(|j| {
                        let n = neighbors_on(&grid, (i, j));
                        n == 3 || n == 2 && grid[i][j]
                    })
                    .collect_vec()
            })
            .collect_vec()
    }
    grid.into_iter().flatten().filter(|a| *a).count()
}

fn part2(input: &str) -> impl Debug {
    let mut grid = grid_with(input, |c| c == '#');
    for _ in 0..100 {
        grid = (0..grid.len())
            .map(|i| {
                (0..grid[i].len())
                    .map(|j| {
                        let n = neighbors_on(&grid, (i, j));
                        n == 3
                            || n == 2 && grid[i][j]
                            || [0, grid.len() - 1].contains(&i)
                                && [0, grid[i].len() - 1].contains(&j)
                    })
                    .collect_vec()
            })
            .collect_vec()
    }
    grid.into_iter().flatten().filter(|a| *a).count()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/18")));
    println!("{:?}", part2(include_str!("../../input/18")));
}
