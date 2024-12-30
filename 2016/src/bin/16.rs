#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn curve(input: &str) -> String {
    let new = input
        .chars()
        .rev()
        .map(|c| if c == '1' { '0' } else { '1' })
        .collect::<String>();

    format!("{input}0{new}")
}

fn cksum(input: &str) -> String {
    input
        .chars()
        .tuples()
        .map(|(a, b)| if a == b { '1' } else { '0' })
        .collect::<String>()
}

fn part1(input: &str) -> impl Debug {
    let disk_len = 272;

    let mut s = input.trim().to_string();
    while s.len() < disk_len {
        s = curve(s.as_str());
    }

    s = s[0..disk_len].to_string();
    while s.len() % 2 == 0 {
        s = cksum(s.as_str());
    }

    s
}

fn part2(input: &str) -> impl Debug {
    let disk_len = 35651584;

    let mut s = input.trim().to_string();
    while s.len() < disk_len {
        s = curve(s.as_str());
    }

    s = s[0..disk_len].to_string();
    while s.len() % 2 == 0 {
        s = cksum(s.as_str());
    }

    s
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/16")));
    println!("{:?}", part2(include_str!("../../input/16")));
}
