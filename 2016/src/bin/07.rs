#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn part1(input: &str) -> impl Debug {
    let re = Regex::new(r"\[(.+?)\]").unwrap();
    input
        .lines()
        .filter(move |s| {
            !re.captures_iter(s).any(|cap| {
                cap[1]
                    .chars()
                    .tuple_windows()
                    .any(|(a, b, c, d)| a == d && b == c && a != b)
            }) && re
                .replace_all(s, "|")
                .chars()
                .tuple_windows()
                .any(|(a, b, c, d)| a == d && b == c && a != b)
        })
        .count()
}

fn part2(input: &str) -> impl Debug {
    let re = Regex::new(r"\[(.+?)\]").unwrap();
    input
        .lines()
        .filter(move |s| {
            let hypernet = re
                .captures_iter(s)
                .map(|cap| cap[1].to_owned())
                .join("|")
                .chars()
                .tuple_windows()
                .filter(|(a, b, c)| a == c && a != b)
                .map(|(a, b, _)| (b, a, b))
                .collect::<HashSet<_>>();
            let supernet = re.replace_all(s, "|");

            supernet
                .chars()
                .tuple_windows()
                .filter(|(a, b, c)| a == c && a != b)
                .any(|a| hypernet.contains(&a))
        })
        .count()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/07")));
    println!("{:?}", part2(include_str!("../../input/07")));
}
