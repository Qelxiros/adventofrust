#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use num::Signed;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn part1(input: &str) -> impl Debug {
    let mut dir = 0;
    let mut dist = (0, 0);
    input.split(", ").for_each(|s| {
        let i = integers_signed::<isize>(s)[0];
        dir += if s.starts_with('R') { 1 } else { 3 };
        dir %= 4;
        dist = (dist.0 + i * DIRS[dir].0, dist.1 + i * DIRS[dir].1);
    });

    dist.0.abs() + dist.1.abs()
}

fn part2(input: &str) -> impl Debug {
    let mut locations = HashSet::new();
    let mut dir = 0;
    let mut loc = (0, 0);
    locations.insert(loc);
    for s in input.split(", ") {
        let i = integers_signed::<isize>(s)[0];
        dir += if s.starts_with('R') { 1 } else { 3 };
        dir %= 4;
        for x in 0..i {
            loc = (loc.0 + DIRS[dir].0, loc.1 + DIRS[dir].1);
            if !locations.insert(loc) {
                return loc.0.abs() + loc.1.abs();
            }
        }
    }

    0
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/01")));
    println!("{:?}", part2(include_str!("../../input/01")));
}
