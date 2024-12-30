#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &str) -> impl Debug {
    input
        .trim()
        .chars()
        .collect_vec()
        .into_iter()
        .circular_tuple_windows()
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a.to_string().parse::<usize>().unwrap())
        .sum::<usize>()
}

fn part2(input: &str) -> impl Debug {
    let v = input.trim().chars().collect_vec();
    let (v1, v2) = v.split_at(input.trim().len() / 2);

    v1.iter()
        .zip(v2)
        .filter(|(a, b)| a == b)
        .map(|(a, b)| {
            a.to_string().parse::<usize>().unwrap()
                + b.to_string().parse::<usize>().unwrap()
        })
        .sum::<usize>()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/01")));
    println!("{:?}", part2(include_str!("../../input/01")));
}
