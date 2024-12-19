#![feature(let_chains)]

use std::collections::*;

use itertools::Itertools;
use regex::Regex;

use adventofrust::utils::*;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn check(pat: &str, valid: &[&str]) -> bool {
    if pat.is_empty() {
        true
    } else {
        valid
            .iter()
            .filter_map(|s| pat.strip_prefix(s))
            .any(|new_pat| check(new_pat, valid))
    }
}

fn part1() -> String {
    let input = include_str!("../../input/19");
    let halves = input.split("\n\n").take(2).collect_vec();
    let valid = halves[0].split(", ").collect_vec();

    halves[1]
        .lines()
        .filter(|s| check(s, &valid))
        .count()
        .to_string()
}

fn check2(pat: &str, valid: &[&str], cache: &mut HashMap<String, usize>) -> usize {
    if let Some(val) = cache.get(pat) {
        return *val;
    }
    let res = if pat.is_empty() {
        1
    } else {
        valid
            .iter()
            .filter_map(|s| pat.strip_prefix(s))
            .map(|new_pat| check2(new_pat, valid, cache))
            .sum()
    };
    cache.insert(pat.to_owned(), res);
    res
}

fn part2() -> String {
    let input = include_str!("../../input/19");

    let halves = input.split("\n\n").take(2).collect_vec();
    let valid = halves[0].split(", ").collect_vec();

    let mut map = HashMap::new();

    halves[1]
        .lines()
        .map(|s| check2(s, &valid, &mut map))
        .sum::<usize>()
        .to_string()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
