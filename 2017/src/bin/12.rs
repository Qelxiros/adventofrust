#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &str) -> impl Debug {
    let map: HashMap<_, _> = input
        .lines()
        .map(|s| s.split_once(" <-> ").unwrap())
        .map(|(a, b)| (a, b.split(", ").collect_vec()))
        .collect();

    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    q.push_back("0");

    while let Some(node) = q.pop_front() {
        if seen.contains(node) {
            continue;
        }
        seen.insert(node);

        for neighbor in &map[node] {
            q.push_back(neighbor);
        }
    }

    seen.len()
}

fn get_group<'a>(
    start: &'a str,
    map: &'a HashMap<&str, Vec<&'a str>>,
) -> HashSet<&'a str> {
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    q.push_back(start);

    while let Some(node) = q.pop_front() {
        if seen.contains(node) {
            continue;
        }
        seen.insert(node);

        for neighbor in &map[node] {
            q.push_back(neighbor);
        }
    }

    seen
}

fn part2(input: &str) -> impl Debug {
    let mut keys = Vec::new();
    let map: HashMap<_, _> = input
        .lines()
        .map(|s| s.split_once(" <-> ").unwrap())
        .map(|(a, b)| {
            keys.push(a);
            (a, b.split(", ").collect_vec())
        })
        .collect();

    let mut seen = HashSet::new();
    let mut groups = 0;
    for k in keys {
        if seen.contains(k) {
            continue;
        }

        seen.extend(get_group(k, &map));
        groups += 1;
    }

    groups
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/12")));
    println!("{:?}", part2(include_str!("../../input/12")));
}
