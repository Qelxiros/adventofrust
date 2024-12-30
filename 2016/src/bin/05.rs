#![feature(let_chains)]

use std::{
    collections::*,
    fmt::{Debug, Write},
};

use common::utils::*;
use itertools::Itertools;
use md5::{Digest, Md5};
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn find_eight(input: &str) -> Vec<u8> {
    let mut v = Vec::new();
    let mut val = 1;
    let hasher = Md5::new_with_prefix(input.trim());
    loop {
        if v.len() == 8 {
            return v;
        }
        let mut h = hasher.clone();
        h.update(val.to_string());
        let h = h.finalize();
        if h[0] == 0 && h[1] == 0 && h[2] < 16 {
            v.push(h[2]);
        }
        val += 1;
    }
}

fn part1(input: &str) -> impl Debug {
    find_eight(input)
        .into_iter()
        .fold(String::new(), |mut acc, i| {
            write!(&mut acc, "{i:x}").unwrap();
            acc
        })
}

fn find_full(input: &str) -> String {
    let mut out = HashMap::new();
    let mut val = 1;
    let hasher = Md5::new_with_prefix(input.trim());
    loop {
        if (0..8).all(|i| out.contains_key(&i)) {
            return (0..8).map(|i| out[&i]).fold(
                String::new(),
                |mut acc, i| {
                    write!(&mut acc, "{i:x}").unwrap();
                    acc
                },
            );
        }
        let mut h = hasher.clone();
        h.update(val.to_string());
        let h = h.finalize();
        if h[0] == 0 && h[1] == 0 && h[2] < 16 {
            out.entry(h[2]).or_insert(h[3] >> 4);
        }
        val += 1;
    }
}

fn part2(input: &str) -> impl Debug {
    find_full(input)
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/05")));
    println!("{:?}", part2(include_str!("../../input/05")));
}
