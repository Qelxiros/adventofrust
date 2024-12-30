#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &str) -> impl Debug {
    input
        .lines()
        .map(|s| {
            let v = integers_unsigned::<usize>(s);
            v.iter().max().unwrap() - v.iter().min().unwrap()
        })
        .sum::<usize>()
}

fn part2(input: &str) -> impl Debug {
    input
        .lines()
        .map(|s| {
            let v = integers_unsigned::<usize>(s);
            v.iter()
                .cartesian_product(&v)
                .find(|&(a, b)| a % b == 0 && a != b)
                .map(|(a, b)| a / b)
                .unwrap()
        })
        .sum::<usize>()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/02")));
    println!("{:?}", part2(include_str!("../../input/02")));
}
