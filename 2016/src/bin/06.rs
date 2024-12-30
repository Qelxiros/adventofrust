#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn part1(input: &str) -> impl Debug {
    (0..input.lines().next().unwrap().len())
        .map(|i| {
            input
                .lines()
                .map(|s| s.chars().nth(i).unwrap())
                .sorted()
                .dedup_with_count()
                .max_by_key(|(c, _)| *c)
                .unwrap()
                .1
        })
        .collect::<String>()
}

fn part2(input: &str) -> impl Debug {
    (0..input.lines().next().unwrap().len())
        .map(|i| {
            input
                .lines()
                .map(|s| s.chars().nth(i).unwrap())
                .sorted()
                .dedup_with_count()
                .min_by_key(|(c, _)| *c)
                .unwrap()
                .1
        })
        .collect::<String>()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/06")));
    println!("{:?}", part2(include_str!("../../input/06")));
}
