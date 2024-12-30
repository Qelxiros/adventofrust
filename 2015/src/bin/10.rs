#![feature(let_chains)]

use std::{
    collections::*,
    fmt::{Debug, Write},
};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn look_say(input: &[char]) -> String {
    let mut out = vec![(input[0], 1)];

    input.iter().skip(1).for_each(|c| {
        if *c == out.last().unwrap().0 {
            out.last_mut().unwrap().1 += 1;
        } else {
            out.push((*c, 1));
        }
    });

    out.into_iter().fold(String::new(), |mut acc, (a, b)| {
        write!(&mut acc, "{b}{a}").unwrap();
        acc
    })
}

fn part1(input: &str) -> impl Debug {
    let mut v = input.trim().chars().collect_vec();
    for i in 0..40 {
        v = look_say(&v).chars().collect_vec();
    }

    v.len()
}

fn part2(input: &str) -> impl Debug {
    let mut v = input.trim().chars().collect_vec();
    for i in 0..50 {
        v = look_say(&v).chars().collect_vec();
    }

    v.len()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/10")));
    println!("{:?}", part2(include_str!("../../input/10")));
}
