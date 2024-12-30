#![feature(let_chains)]

use std::{collections::*, f64::consts::PI, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn process(house: usize) -> usize {
    (1..=((house as f64).sqrt() as usize))
        .filter(|i| house % i == 0)
        .map(|i| (i + if i == house / i { 0 } else { house / i }) * 10)
        .sum::<usize>()
}

fn process2(house: usize) -> usize {
    (1..=((house as f64).sqrt() as usize))
        .filter(|i| house % i == 0)
        .map(|i| {
            ((if house / i > 50 { 0 } else { i })
                + if i == house / i || i > 50 {
                    0
                } else {
                    house / i
                })
                * 11
        })
        .sum::<usize>()
}

fn part1(input: &str) -> impl Debug {
    let input = input.trim().parse::<usize>().unwrap();
    (1..usize::MAX).find(|i| process(*i) > input).unwrap()
}

fn part2(input: &str) -> impl Debug {
    let input = input.trim().parse::<usize>().unwrap();
    (1..usize::MAX).find(|i| process2(*i) > input).unwrap()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/20")));
    println!("{:?}", part2(include_str!("../../input/20")));
}
