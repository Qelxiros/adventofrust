#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn part1(input: &str) -> impl Debug {
    integers_unsigned::<usize>(input)
        .into_iter()
        .powerset()
        .filter(|v| v.iter().sum::<usize>() == 150)
        .count()
}

fn part2(input: &str) -> impl Debug {
    integers_unsigned::<usize>(input)
        .into_iter()
        .powerset()
        .filter(|v| v.iter().sum::<usize>() == 150)
        .map(|v| v.len())
        .min_set()
        .len()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/17")));
    println!("{:?}", part2(include_str!("../../input/17")));
}
