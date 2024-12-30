#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &str) -> impl Debug {
    input
        .lines()
        .filter(|s| s.split_whitespace().all_unique())
        .count()
}

fn part2(input: &str) -> impl Debug {
    input
        .lines()
        .filter(|s| {
            s.split_whitespace()
                .map(|s| s.chars().sorted().collect::<String>())
                .all_unique()
        })
        .count()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/04")));
    println!("{:?}", part2(include_str!("../../input/04")));
}
