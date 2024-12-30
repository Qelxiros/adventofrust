#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &str) -> impl Debug {
    let step = integers_unsigned::<usize>(input)[0];
    let mut v = vec![0];
    let mut pos = 0;

    for i in 1..=2017 {
        pos += step + 1;
        pos %= v.len();
        v.insert(pos, i);
    }

    v[(pos + 1) % v.len()]
}

fn part2(input: &str) -> impl Debug {
    let step = integers_unsigned::<usize>(input)[0];
    let mut len = 1;
    let mut pos = 0;

    let mut out = 0;

    for i in 1..=50_000_000 {
        pos += step;
        pos %= len;
        pos += 1;
        len += 1;
        if pos == 1 {
            out = i;
        }
    }

    out
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/17")));
    println!("{:?}", part2(include_str!("../../input/17")));
}
