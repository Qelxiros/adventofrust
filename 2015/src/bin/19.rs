#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn part1(input: &str) -> impl Debug {
    let halves = input.split("\n\n").take(2).collect_vec();
    let mut set = HashSet::new();
    halves[0]
        .lines()
        .map(|s| s.split_once(" => ").unwrap())
        .for_each(|(find, replace)| {
            halves[1].match_indices(find).for_each(|(idx, _)| {
                set.insert(format!(
                    "{}{}{}",
                    &halves[1][..idx],
                    replace,
                    &halves[1][idx + find.len()..]
                ));
            })
        });

    set.len()
}

fn dfs(s: String, target: &str, t: &[(&str, &str)]) -> Option<usize> {
    if s == target {
        return Some(0);
    }
    for (replace, find) in t {
        for (idx, _) in s.match_indices(find) {
            if let Some(v) = dfs(
                format!(
                    "{}{}{}",
                    &s[..idx],
                    replace,
                    if idx + find.len() < s.len() {
                        &s[idx + find.len()..]
                    } else {
                        ""
                    }
                ),
                target,
                t,
            ) {
                return Some(v + 1);
            }
        }
    }

    None
}

fn part2(input: &str) -> impl Debug {
    let halves = input.split("\n\n").take(2).collect_vec();
    let t = halves[0]
        .lines()
        .map(|s| s.split_once(" => ").unwrap())
        .collect_vec();

    dfs(halves[1].trim().to_string(), "e", &t).unwrap()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/19")));
    println!("{:?}", part2(include_str!("../../input/19")));
}
