#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &str) -> impl Debug {
    let mut map: HashMap<&str, usize> = input
        .trim()
        .split(',')
        .sorted()
        .dedup_with_count()
        .map(|(a, b)| (b, a))
        .collect();

    for key in ["n", "se", "ne", "s", "nw", "sw"] {
        map.entry(key).or_insert(0);
    }

    for pair in [
        ("n", "se", "ne"),
        ("n", "sw", "nw"),
        ("s", "ne", "se"),
        ("s", "nw", "sw"),
        ("ne", "nw", "n"),
        ("se", "sw", "s"),
    ] {
        let diff = map[pair.0].min(map[pair.1]);
        map.entry(pair.0).and_modify(|v| *v -= diff);
        map.entry(pair.1).and_modify(|v| *v -= diff);
        map.entry(pair.2).and_modify(|v| *v += diff);
    }

    for pair in [("n", "s"), ("ne", "sw"), ("nw", "se")] {
        let diff = map[pair.0].min(map[pair.1]);
        map.entry(pair.0).and_modify(|v| *v -= diff);
        map.entry(pair.1).and_modify(|v| *v -= diff);
    }

    map.values().sum::<usize>()
}

fn get_dist(map: &mut HashMap<&str, usize>) -> usize {
    for pair in [
        ("n", "se", "ne"),
        ("n", "sw", "nw"),
        ("s", "ne", "se"),
        ("s", "nw", "sw"),
        ("ne", "nw", "n"),
        ("se", "sw", "s"),
    ] {
        let diff = map[pair.0].min(map[pair.1]);
        map.entry(pair.0).and_modify(|v| *v -= diff);
        map.entry(pair.1).and_modify(|v| *v -= diff);
        map.entry(pair.2).and_modify(|v| *v += diff);
    }

    for pair in [("n", "s"), ("ne", "sw"), ("nw", "se")] {
        let diff = map[pair.0].min(map[pair.1]);
        map.entry(pair.0).and_modify(|v| *v -= diff);
        map.entry(pair.1).and_modify(|v| *v -= diff);
    }

    map.values().sum::<usize>()
}

fn part2(input: &str) -> impl Debug {
    let mut map: HashMap<&str, usize> = HashMap::new();

    for key in ["n", "se", "ne", "s", "nw", "sw"] {
        map.entry(key).or_insert(0);
    }

    let mut max = 0;
    for dir in input.trim().split(',') {
        *map.entry(dir).or_insert(0) += 1;
        max = max.max(get_dist(&mut map));
    }

    max
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/11")));
    println!("{:?}", part2(include_str!("../../input/11")));
}
