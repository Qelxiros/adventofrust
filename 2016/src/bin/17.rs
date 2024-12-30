#![feature(let_chains)]

use std::{
    collections::*,
    fmt::{Debug, Write},
};

use common::utils::*;
use itertools::Itertools;
use md5::{Digest, Md5};
use regex::Regex;

const DIRS: [((isize, isize), char); 4] =
    [((0, -1), 'U'), ((0, 1), 'D'), ((-1, 0), 'L'), ((1, 0), 'R')];

fn part1(input: &str) -> impl Debug {
    let start = ((0, 0), String::new());
    let target = (3, 3);
    let hasher = Md5::new_with_prefix(input.trim());

    let mut q = VecDeque::new();
    q.push_back(start);

    while let Some((pos, steps)) = q.pop_front() {
        if pos == target {
            return steps;
        }

        for dir in hasher
            .clone()
            .chain_update(&steps)
            .finalize()
            .into_iter()
            .take(2)
            .flat_map(|v| vec![v >> 4 > 10, v & 0xf > 10])
            .zip(DIRS)
            .filter(|(c, _)| *c)
            .map(|(_, d)| d)
            .filter(|d| {
                pos.0 + d.0.0 >= 0
                    && pos.0 + d.0.0 < 4
                    && pos.1 + d.0.1 >= 0
                    && pos.1 + d.0.1 < 4
            })
        {
            q.push_back((
                (pos.0 + dir.0.0, pos.1 + dir.0.1),
                format!("{steps}{}", dir.1),
            ));
        }
    }

    String::new()
}

fn part2(input: &str) -> impl Debug {
    let start = ((0, 0), String::new());
    let target = (3, 3);
    let hasher = Md5::new_with_prefix(input.trim());

    let mut q = VecDeque::new();
    q.push_back(start);

    let mut max = 0;
    while let Some((pos, steps)) = q.pop_front() {
        if pos == target {
            max = max.max(steps.len());
            continue;
        }

        // println!("--- {steps}");
        for dir in hasher
            .clone()
            .chain_update(&steps)
            .finalize()
            .into_iter()
            .take(2)
            .flat_map(|v| vec![v >> 4 > 10, v & 0xf > 10])
            .zip(DIRS)
            .filter(|(c, _)| *c)
            .map(|(_, d)| d)
            .filter(|d| {
                pos.0 + d.0.0 >= 0
                    && pos.0 + d.0.0 < 4
                    && pos.1 + d.0.1 >= 0
                    && pos.1 + d.0.1 < 4
            })
        {
            // println!("{dir:?}");
            q.push_back((
                (pos.0 + dir.0.0, pos.1 + dir.0.1),
                format!("{steps}{}", dir.1),
            ));
        }
    }

    max
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/17")));
    println!("{:?}", part2(include_str!("../../input/17")));
}
