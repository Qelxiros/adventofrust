#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use md5::{Digest, Md5};
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn part1(input: &str) -> impl Debug {
    let mut val = 1;
    let hasher = Md5::new_with_prefix(input.trim());
    loop {
        let mut h = hasher.clone();
        h.update(val.to_string());
        let h = h.finalize();
        if h[0] == 0 && h[1] == 0 && h[2] < 16 {
            return val;
        }
        val += 1;
    }
}

fn part2(input: &str) -> impl Debug {
    let mut val = 1;
    let hasher = Md5::new_with_prefix(input.trim());
    loop {
        let mut h = hasher.clone();
        h.update(val.to_string());
        let h = h.finalize();
        if h[0] == 0 && h[1] == 0 && h[2] == 0 {
            return val;
        }
        val += 1;
    }
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/04")));
    println!("{:?}", part2(include_str!("../../input/04")));
}
