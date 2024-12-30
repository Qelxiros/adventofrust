#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn part1(input: &str) -> impl Debug {
    let mut grid = [[false; 1000]; 1000];

    for line in input.lines() {
        let vals = integers_unsigned::<usize>(line);
        match line.chars().nth(6) {
            Some('n') => {
                for i in vals[0]..=vals[2] {
                    for j in vals[1]..=vals[3] {
                        grid[i][j] = true
                    }
                }
            }
            Some('f') => {
                for i in vals[0]..=vals[2] {
                    for j in vals[1]..=vals[3] {
                        grid[i][j] = false
                    }
                }
            }
            Some(' ') => {
                for i in vals[0]..=vals[2] {
                    for j in vals[1]..=vals[3] {
                        grid[i][j] = !grid[i][j]
                    }
                }
            }
            _ => {}
        }
    }

    grid.into_iter()
        .map(|row| row.into_iter().filter(|a| *a).count())
        .sum::<usize>()
}

fn part2(input: &str) -> impl Debug {
    let mut grid = vec![vec![0usize; 1000]; 1000];

    for line in input.lines() {
        let vals = integers_unsigned::<usize>(line);
        match line.chars().nth(6) {
            Some('n') => {
                for i in vals[0]..=vals[2] {
                    for j in vals[1]..=vals[3] {
                        grid[i][j] += 1
                    }
                }
            }
            Some('f') => {
                for i in vals[0]..=vals[2] {
                    for j in vals[1]..=vals[3] {
                        grid[i][j] = grid[i][j].saturating_sub(1)
                    }
                }
            }
            Some(' ') => {
                for i in vals[0]..=vals[2] {
                    for j in vals[1]..=vals[3] {
                        grid[i][j] += 2
                    }
                }
            }
            _ => {}
        }
    }

    grid.into_iter()
        .map(|row| row.into_iter().sum::<usize>())
        .sum::<usize>()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/06")));
    println!("{:?}", part2(include_str!("../../input/06")));
}
