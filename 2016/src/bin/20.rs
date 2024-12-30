#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &str) -> impl Debug {
    let nums = input
        .lines()
        .map(integers_unsigned::<usize>)
        .sorted_by_key(|v| v[0])
        .collect_vec();

    let mut max = 0;

    for v in nums {
        if v[0] > max + 1 {
            return max + 1;
        }
        max = max.max(v[1]);
    }

    0
}

fn part2(input: &str) -> impl Debug {
    let nums = input
        .lines()
        .map(integers_unsigned::<usize>)
        .sorted_by_key(|v| v[0])
        .collect_vec();

    let mut max = 0;
    let mut total = 0;

    for v in nums {
        if v[0] > max + 1 {
            total += v[0] - max - 1;
        }
        max = max.max(v[1]);
    }

    total
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/20")));
    println!("{:?}", part2(include_str!("../../input/20")));
}
