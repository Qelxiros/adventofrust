#![feature(let_chains)]

use std::{
    collections::*,
    fmt::{Debug, Write},
};

use common::utils::*;
use itertools::Itertools;
use md5::{Digest, Md5};
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn hash(
    mut hasher: Md5,
    suffix: usize,
    p2: bool,
    cache: &mut HashMap<String, String>,
) -> String {
    hasher.update(suffix.to_string().as_bytes());
    let s = hasher
        .finalize()
        .into_iter()
        .fold(String::new(), |mut acc, e| {
            write!(&mut acc, "{e:0>2x}").unwrap();
            acc
        });
    if p2 { hash2(s, cache) } else { s }
}

fn check(
    hasher: Md5,
    suffix: usize,
    p2: bool,
    cache: &mut HashMap<String, String>,
) -> bool {
    hash(hasher.clone(), suffix, p2, cache)
        .chars()
        .tuple_windows()
        .find(|(a, b, c)| a == b && a == c)
        .map(|(x, _, _)| {
            (1..=1000).any(|i| {
                let h = hash(hasher.clone(), suffix + i, p2, cache);
                let res = h.chars().tuple_windows().any(|(a, b, c, d, e)| {
                    a == x && b == x && c == x && d == x && e == x
                });
                res
            })
        })
        .unwrap_or(false)
}

fn part1(input: &str) -> impl Debug {
    let hasher = Md5::new_with_prefix(input.trim());
    let mut cache = HashMap::new();
    (0..usize::MAX)
        .filter(|i| check(hasher.clone(), *i, false, &mut cache))
        .nth(63)
        .unwrap()
}

fn hash2(original: String, cache: &mut HashMap<String, String>) -> String {
    let mut s = original.clone();
    if let Some(out) = cache.get(&s) {
        return out.clone();
    }
    for _ in 0..2016 {
        let hasher = Md5::new_with_prefix(s);
        s = hasher
            .finalize()
            .into_iter()
            .fold(String::new(), |mut acc, e| {
                write!(&mut acc, "{e:0>2x}").unwrap();
                acc
            })
    }

    cache.insert(original, s.clone());
    s
}

fn part2(input: &str) -> impl Debug {
    let hasher = Md5::new_with_prefix(input.trim());
    let mut cache = HashMap::new();
    (0..usize::MAX)
        .filter(|i| check(hasher.clone(), *i, true, &mut cache))
        .nth(63)
        .unwrap()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/14")));
    println!("{:?}", part2(include_str!("../../input/14")));
}
