#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &str) -> impl Debug {
    let mut cur = 5;
    let mut out = 0;
    input.lines().for_each(|s| {
        for c in s.chars() {
            match c {
                'U' => {
                    if cur > 3 {
                        cur -= 3
                    }
                }
                'D' => {
                    if cur <= 6 {
                        cur += 3
                    }
                }
                'R' => {
                    if cur % 3 != 0 {
                        cur += 1
                    }
                }
                'L' => {
                    if cur % 3 != 1 {
                        cur -= 1
                    }
                }
                _ => panic!(),
            }
        }
        out *= 10;
        out += cur;
    });
    out
}

fn part2(input: &str) -> impl Debug {
    let mut cur = (2, 0);
    let mut out = String::new();
    let grid = grid(
        "  1
 234
56789
 ABC
  D",
    );
    input.lines().for_each(|s| {
        for c in s.chars() {
            let dir = match c {
                'U' => DIRS[0],
                'R' => DIRS[1],
                'D' => DIRS[2],
                'L' => DIRS[3],
                _ => panic!(),
            };

            if let Some(c) = grid
                .get((cur.0 + dir.0) as usize)
                .and_then(|r| r.get((cur.1 + dir.1) as usize))
            {
                if *c != ' ' {
                    cur = (cur.0 + dir.0, cur.1 + dir.1);
                }
            }
        }
        out.push(grid[cur.0 as usize][cur.1 as usize]);
    });

    out
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/02")));
    println!("{:?}", part2(include_str!("../../input/02")));
}
