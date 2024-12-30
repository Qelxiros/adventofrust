#![feature(let_chains)]

use std::{collections::*, fmt::Debug, iter::once};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn next(cur: Vec<bool>) -> Vec<bool> {
    once(true)
        .chain(cur)
        .chain(once(true))
        .tuple_windows()
        .map(|(a, _, c)| a == c)
        .collect_vec()
}

fn print(g: &[Vec<bool>]) {
    for v in g {
        println!("{}", v.iter().map(|b| if *b { '.' } else { '^' }).join(""));
    }
}

fn part1(input: &str) -> impl Debug {
    let mut grid = grid_with(input, |c| c == '.');

    for i in 1..40 {
        grid.push(next(grid.last().unwrap().to_vec()));
    }

    grid.into_iter().flatten().filter(|a| *a).count()
}

fn part2(input: &str) -> impl Debug {
    let mut grid = grid_with(input, |c| c == '.');

    for i in 1..400000 {
        grid.push(next(grid.last().unwrap().to_vec()));
    }

    grid.into_iter().flatten().filter(|a| *a).count()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/18")));
    println!("{:?}", part2(include_str!("../../input/18")));
}
