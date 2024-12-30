#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &str) -> &str {
    let map = input
        .lines()
        .filter_map(|s| s.split_once(" -> "))
        .flat_map(|(left, right)| {
            let l = left.split_once(' ').unwrap().0;
            right.split(", ").map(move |r| (r, l))
        })
        .collect::<HashMap<_, _>>();

    let mut val = map.values().next().cloned().unwrap();

    while let Some(v) = map.get(val) {
        val = v;
    }

    val
}

fn get_total_weight<'a>(
    root: &'a str,
    map: &HashMap<&'a str, (usize, Vec<&'a str>)>,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(&v) = cache.get(root) {
        return v;
    }

    let (weight, children) = &map[root];
    let w = weight
        + children
            .iter()
            .map(|c| get_total_weight(c, map, cache))
            .sum::<usize>();
    cache.insert(root, w);
    w
}

fn check(
    root: &str,
    map: &HashMap<&'static str, (usize, Vec<&'static str>)>,
    cache: &mut HashMap<&str, usize>,
) -> Option<(String, usize)> {
    let (weight, children) = map[root].clone();

    let weights = children
        .iter()
        .map(|c| get_total_weight(c, map, cache))
        .collect_vec();

    weights
        .iter()
        .position(|e| {
            let mut w = weights.clone();
            w.retain(|a| a != e);
            w.len() == weights.len() - 1
        })
        .map(|idx| {
            (
                children[idx].to_string(),
                map[children[idx]].0 + weights[(idx + 1) % weights.len()]
                    - weights[idx],
            )
        })
}

fn part2(input: &'static str) -> impl Debug {
    let mut map = HashMap::new();
    for line in input.lines() {
        if let Some((left, right)) = line.split_once(" -> ") {
            let l = left.split_once(' ').unwrap().0;
            let weight = integers_unsigned::<usize>(left)[0];
            map.insert(l, (weight, right.split(", ").collect_vec()));
        } else {
            let word = line.split_whitespace().next().unwrap();
            let weight = integers_unsigned::<usize>(line)[0];
            map.insert(word, (weight, Vec::new()));
        }
    }

    let mut root = part1(input).to_string();
    let mut cache = HashMap::new();
    let mut new = 0;

    while let Some((s, n)) = check(root.as_str(), &map, &mut cache) {
        root = s;
        new = n;
    }

    new
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/07")));
    println!("{:?}", part2(include_str!("../../input/07")));
}
