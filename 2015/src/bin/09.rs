#![feature(let_chains)]

use std::{cmp::Reverse, collections::*, fmt::Debug, hash::Hash};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn part1(input: &str) -> impl Debug {
    let mut graph = HashMap::new();
    let mut cities = HashSet::new();
    for line in input.lines() {
        let words = line.split_whitespace().collect_vec();
        let dist = words[4].parse::<usize>().unwrap();
        graph.insert((words[0], words[2]), dist);
        graph.insert((words[2], words[0]), dist);
        cities.insert(words[0]);
        cities.insert(words[2]);
    }

    cities
        .clone()
        .into_iter()
        .permutations(cities.len())
        .map(|p| {
            p.into_iter()
                .tuple_windows()
                .map(|(a, b)| graph[&(a, b)])
                .sum::<usize>()
        })
        .min()
        .unwrap()
}

fn part2(input: &str) -> impl Debug {
    let mut graph = HashMap::new();
    let mut cities = HashSet::new();
    for line in input.lines() {
        let words = line.split_whitespace().collect_vec();
        let dist = words[4].parse::<usize>().unwrap();
        graph.insert((words[0], words[2]), dist);
        graph.insert((words[2], words[0]), dist);
        cities.insert(words[0]);
        cities.insert(words[2]);
    }

    cities
        .clone()
        .into_iter()
        .permutations(cities.len())
        .map(|p| {
            p.into_iter()
                .tuple_windows()
                .map(|(a, b)| graph[&(a, b)])
                .sum::<usize>()
        })
        .max()
        .unwrap()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/09")));
    println!("{:?}", part2(include_str!("../../input/09")));
}
