#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn part1(input: &str) -> impl Debug {
    input
        .lines()
        .filter(|s| s.chars().filter(|c| "aieou".contains(*c)).count() >= 3)
        .filter(|s| s.chars().tuple_windows().any(|(a, b)| a == b))
        .filter(|s| {
            !s.contains("ab")
                && !s.contains("cd")
                && !s.contains("pq")
                && !s.contains("xy")
        })
        .count()
}

fn part2(input: &str) -> impl Debug {
    input
        .lines()
        .filter(|s| {
            s.chars()
                .tuple_windows::<(_, _)>()
                .enumerate()
                .any(|(idx, a)| {
                    let x = s.chars().tuple_windows::<(_, _)>().collect_vec();
                    x.into_iter()
                        .rposition(|b| a == b)
                        .is_some_and(|r| r > idx + 1)
                })
        })
        .filter(|s| s.chars().tuple_windows().any(|(a, _, c)| a == c))
        .count()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/05")));
    println!("{:?}", part2(include_str!("../../input/05")));
}
