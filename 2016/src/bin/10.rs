#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[derive(PartialEq, Eq, Hash)]
enum Dest {
    Bot(usize),
    Output(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rule {
    Value(usize),
    LowFrom(usize),
    HighFrom(usize),
}

fn get_val(
    rule: Rule,
    map: &HashMap<Dest, Vec<Rule>>,
    cache: &mut HashMap<Rule, usize>,
) -> usize {
    if let Some(v) = cache.get(&rule) {
        return *v;
    }
    let res = match rule {
        Rule::Value(v) => v,
        Rule::LowFrom(v) => map
            .get(&Dest::Bot(v))
            .unwrap()
            .iter()
            .map(|r| get_val(*r, map, cache))
            .min()
            .unwrap(),
        Rule::HighFrom(v) => map
            .get(&Dest::Bot(v))
            .unwrap()
            .iter()
            .map(|r| get_val(*r, map, cache))
            .max()
            .unwrap(),
    };
    cache.insert(rule, res);
    res
}

fn part1(input: &str) -> impl Debug {
    let mut map = HashMap::new();

    for line in input.lines() {
        let i = integers_unsigned(line);
        if line.starts_with('v') {
            map.entry(Dest::Bot(i[1]))
                .or_insert_with(Vec::new)
                .push(Rule::Value(i[0]));
        } else {
            let words = line.split_whitespace().collect_vec();
            if words[5].starts_with('b') {
                map.entry(Dest::Bot(i[1]))
                    .or_insert_with(Vec::new)
                    .push(Rule::LowFrom(i[0]));
            } else {
                map.insert(Dest::Output(i[1]), vec![
                    Rule::LowFrom(i[0]),
                    Rule::LowFrom(i[0]),
                ]);
            }
            if words[10].starts_with('b') {
                map.entry(Dest::Bot(i[2]))
                    .or_insert_with(Vec::new)
                    .push(Rule::HighFrom(i[0]));
            } else {
                map.insert(Dest::Output(i[2]), vec![
                    Rule::HighFrom(i[0]),
                    Rule::HighFrom(i[0]),
                ]);
            }
        }
    }

    let mut cache = HashMap::new();

    (0..usize::MAX)
        .find(|&i| {
            get_val(Rule::LowFrom(i), &map, &mut cache) == 17
                && get_val(Rule::HighFrom(i), &map, &mut cache) == 61
        })
        .unwrap()
}

fn get_output(
    out: usize,
    map: &HashMap<Dest, Vec<Rule>>,
    cache: &mut HashMap<Rule, usize>,
) -> usize {
    get_val(map[&Dest::Output(out)][0], map, cache)
}

fn part2(input: &str) -> impl Debug {
    let mut map = HashMap::new();

    for line in input.lines() {
        let i = integers_unsigned(line);
        if line.starts_with('v') {
            map.entry(Dest::Bot(i[1]))
                .or_insert_with(Vec::new)
                .push(Rule::Value(i[0]));
        } else {
            let words = line.split_whitespace().collect_vec();
            if words[5].starts_with('b') {
                map.entry(Dest::Bot(i[1]))
                    .or_insert_with(Vec::new)
                    .push(Rule::LowFrom(i[0]));
            } else {
                map.insert(Dest::Output(i[1]), vec![
                    Rule::LowFrom(i[0]),
                    Rule::LowFrom(i[0]),
                ]);
            }
            if words[10].starts_with('b') {
                map.entry(Dest::Bot(i[2]))
                    .or_insert_with(Vec::new)
                    .push(Rule::HighFrom(i[0]));
            } else {
                map.insert(Dest::Output(i[2]), vec![
                    Rule::HighFrom(i[0]),
                    Rule::HighFrom(i[0]),
                ]);
            }
        }
    }

    let mut cache = HashMap::new();

    get_output(0, &map, &mut cache)
        * get_output(1, &map, &mut cache)
        * get_output(2, &map, &mut cache)
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/10")));
    println!("{:?}", part2(include_str!("../../input/10")));
}
