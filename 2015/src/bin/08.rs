#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn part1(input: &str) -> impl Debug {
    let re = Regex::new(r"\\(.)").unwrap();
    input
        .lines()
        .map(|line| {
            re.captures_iter(line)
                .map(|c| {
                    let s = c[1].to_string();
                    match s.as_str() {
                        r"\" => 1,
                        r#"""# => 1,
                        "x" => 3,
                        _ => 0,
                    }
                })
                .sum::<usize>()
                + 2
        })
        .sum::<usize>()
}

fn part2(input: &str) -> impl Debug {
    input
        .lines()
        .map(|line| line.chars().filter(|c| r#"\""#.contains(*c)).count() + 2)
        .sum::<usize>()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/08")));
    println!("{:?}", part2(include_str!("../../input/08")));
}
