#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use itertools::Itertools;
use regex::Regex;

use adventofrust::utils::*;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn part1(input: &str) -> impl Debug {
    let blocks = input.split("\n\n");
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for b in blocks {
        let g = grid(b);
        if g[0][0] == '.' {
            let key = (0..5)
                .map(|idx| {
                    g.iter()
                        .map(|row| row[idx])
                        .skip_while(|c| *c == '.')
                        .count()
                        - 1
                })
                .collect_vec();
            keys.push(key);
        }
        if g[0][0] == '#' {
            let lock = (0..5)
                .map(|idx| {
                    g.iter()
                        .map(|row| row[idx])
                        .take_while(|c| *c == '#')
                        .count()
                        - 1
                })
                .collect_vec();
            locks.push(lock);
        }
    }

    locks
        .into_iter()
        .cartesian_product(keys)
        .filter(|(lock, key)| lock.iter().zip(key).all(|(a, b)| a + b <= 5))
        .count()
}

fn part2(input: &str) -> impl Debug {}

fn main() {
    println!("{:?}", part1(include_str!("../../input/25")));
    println!("{:?}", part2(include_str!("../../input/25")));
}
