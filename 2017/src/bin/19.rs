#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &str) -> impl Debug {
    let g = grid(input);
    let mut pos = (0, g[0].iter().position(|c| *c == '|').unwrap() as isize);
    let mut dir = 2;
    let mut out = String::new();
    loop {
        if g[pos.0 as usize][pos.1 as usize] == '+' {
            for d in 0..4 {
                if d == (dir + 2) % 4 {
                    continue;
                }
                if g[(pos.0 + DIRS[d].0) as usize][(pos.1 + DIRS[d].1) as usize]
                    != ' '
                {
                    dir = d;
                    break;
                }
            }
        }
        if !"|-+".contains(g[pos.0 as usize][pos.1 as usize]) {
            let c = g[pos.0 as usize][pos.1 as usize];
            if c == ' ' {
                break;
            }
            out.push(c);
        }
        pos = add2(pos, DIRS[dir]);
    }

    out
}

fn part2(input: &str) -> impl Debug {
    let g = grid(input);
    let mut pos = (0, g[0].iter().position(|c| *c == '|').unwrap() as isize);
    let mut dir = 2;
    let mut out = 0;
    loop {
        if g[pos.0 as usize][pos.1 as usize] == ' ' {
            break;
        }
        if g[pos.0 as usize][pos.1 as usize] == '+' {
            for d in 0..4 {
                if d == (dir + 2) % 4 {
                    continue;
                }
                if g[(pos.0 + DIRS[d].0) as usize][(pos.1 + DIRS[d].1) as usize]
                    != ' '
                {
                    dir = d;
                    break;
                }
            }
        }
        pos = add2(pos, DIRS[dir]);
        out += 1;
    }

    out
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/19")));
    println!("{:?}", part2(include_str!("../../input/19")));
}
