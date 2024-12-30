#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn calc(
    wire: &str,
    map: &HashMap<&str, Vec<&str>>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if let Some(v) = cache.get(wire) {
        return *v;
    }
    let Some(v) = map.get(&wire) else { panic!() };
    if v.len() == 1 {
        return v[0].parse().unwrap_or_else(|_| calc(v[0], map, cache));
    }
    if v.len() == 2 {
        return !calc(v[1], map, cache);
    }
    let a = v[0]
        .parse::<usize>()
        .unwrap_or_else(|_| calc(v[0], map, cache));
    let b = v[2]
        .parse::<usize>()
        .unwrap_or_else(|_| calc(v[2], map, cache));

    let res = match v[1].chars().next().unwrap() {
        'L' => a << b,
        'R' => a >> b,
        'A' => a & b,
        'O' => a | b,
        _ => panic!(),
    };
    cache.insert(wire.to_owned(), res);
    res
}

fn part1(input: &str) -> usize {
    let map = input
        .lines()
        .map(|s| {
            let words = s.split_whitespace().collect_vec();
            (
                words.last().copied().unwrap(),
                words
                    .into_iter()
                    .take_while(|w| !w.starts_with('-'))
                    .collect_vec(),
            )
        })
        .collect::<HashMap<_, _>>();

    let mut cache = HashMap::new();
    calc("a", &map, &mut cache)
}

fn part2(input: &str) -> impl Debug {
    let mut map = input
        .lines()
        .map(|s| {
            let words = s.split_whitespace().collect_vec();
            (
                words.last().copied().unwrap(),
                words
                    .into_iter()
                    .take_while(|w| !w.starts_with('-'))
                    .collect_vec(),
            )
        })
        .collect::<HashMap<_, _>>();

    let s = part1(input).to_string();
    map.insert("b", vec![s.as_str()]);

    let mut cache = HashMap::new();
    calc("a", &map, &mut cache)
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/07")));
    println!("{:?}", part2(include_str!("../../input/07")));
}
