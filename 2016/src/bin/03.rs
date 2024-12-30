#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn part1(input: &str) -> impl Debug {
    input
        .lines()
        .map(integers_unsigned::<usize>)
        .map(|v| v.into_iter().sorted().collect_vec())
        .filter(|v| v[0] + v[1] > v[2])
        .count()
}

fn part2(input: &str) -> impl Debug {
    input
        .lines()
        .tuples()
        .flat_map(|(a, b, c)| {
            let a = integers_unsigned(a);
            let b = integers_unsigned(b);
            let c = integers_unsigned(c);
            (0..3).map(move |i| vec![a[i], b[i], c[i]])
        })
        .map(|v| v.into_iter().sorted().collect::<Vec<usize>>())
        .filter(|v| v[0] + v[1] > v[2])
        .count()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/03")));
    println!("{:?}", part2(include_str!("../../input/03")));
}
