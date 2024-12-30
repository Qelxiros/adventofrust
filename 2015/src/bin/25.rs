#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn part1(input: &str) -> impl Debug {
    let i = integers_unsigned::<usize>(input);
    let row = i[0];
    let col = i[1];
    let diag = row + col - 2;
    let idx = col + diag * (diag + 1) / 2;
    let mut code = 20151125usize;
    for _ in 1..idx {
        code *= 252533;
        code %= 33554393;
    }

    code
}

fn part2(input: &str) -> impl Debug {}

fn main() {
    println!("{:?}", part1(include_str!("../../input/25")));
    println!("{:?}", part2(include_str!("../../input/25")));
}
