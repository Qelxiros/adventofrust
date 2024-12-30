#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &str) -> impl Debug {
    let re = Regex::new("!.").unwrap();
    let input = re.replace_all(input.trim(), "");
    let re = Regex::new(r"<[^>]*>").unwrap();
    let input = re.replace_all(&input, "");

    let mut score = 0;
    let mut total = 0;
    input.chars().for_each(|c| {
        if c == '{' {
            score += 1;
            total += score;
        } else if c == '}' {
            score -= 1
        }
    });

    total
}

fn part2(input: &str) -> impl Debug {
    let re = Regex::new("!.").unwrap();
    let input = re.replace_all(input.trim(), "");
    let old_len = input.len();
    let re = Regex::new(r"<[^>]*>").unwrap();
    let matches = re.find_iter(&input).count();
    let input = re.replace_all(&input, "");

    old_len - input.len() - 2 * matches
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/09")));
    println!("{:?}", part2(include_str!("../../input/09")));
}
