#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &str) -> impl Debug {
    let mut max = 0;
    let map: HashMap<_, _> = input
        .lines()
        .map(integers_unsigned::<usize>)
        .map(|v| {
            max = max.max(v[0]);
            (v[0], v[1])
        })
        .collect();

    let mut total = 0;
    for depth in 0..=max {
        if let Some(range) = map.get(&depth)
            && depth % (range * 2 - 2) == 0
        {
            total += depth * range
        }
    }

    total
}

fn part2(input: &str) -> impl Debug {
    let mut max = 0;
    let map: HashMap<_, _> = input
        .lines()
        .map(integers_unsigned::<usize>)
        .map(|v| {
            max = max.max(v[0]);
            (v[0], v[1])
        })
        .collect();

    (0..usize::MAX)
        .find(|delay| {
            (0..=max).all(|depth| {
                map.get(&depth)
                    .is_none_or(|range| (depth + delay) % (range * 2 - 2) != 0)
            })
        })
        .unwrap()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/13")));
    println!("{:?}", part2(include_str!("../../input/13")));
}
