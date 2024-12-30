#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn cycle(banks: &mut [usize]) {
    let idx = banks
        .iter()
        .position(|e| e == banks.iter().max().unwrap())
        .unwrap();

    let blocks = banks[idx];
    banks[idx] = 0;

    let mut i = (idx + 1).rem_euclid(banks.len());
    for _ in 0..blocks {
        banks[i] += 1;
        i = (i + 1).rem_euclid(banks.len());
    }
}

fn part1(input: &str) -> impl Debug {
    let mut seen = HashSet::new();
    let mut steps = 0;
    let mut v = integers_unsigned::<usize>(input);
    loop {
        steps += 1;
        cycle(&mut v);
        if !seen.insert(v.clone()) {
            return steps;
        }
    }
}

fn part2(input: &str) -> impl Debug {
    let mut seen = HashMap::new();
    let mut steps = 0usize;
    let mut v = integers_unsigned::<usize>(input);
    loop {
        steps += 1;
        cycle(&mut v);
        if let Some(prev) = seen.get(&v) {
            return steps - prev;
        }
        seen.insert(v.clone(), steps);
    }
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/06")));
    println!("{:?}", part2(include_str!("../../input/06")));
}
