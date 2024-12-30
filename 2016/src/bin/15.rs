#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use num::traits::Euclid;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &str) -> impl Debug {
    let (vals, moduli): (Vec<_>, Vec<_>) = input
        .lines()
        .map(integers_signed::<isize>)
        .map(|v| (v[1], (v[1] - v[3] - v[0]).rem_euclid(v[1])))
        .unzip();
    crt(&vals, &moduli)
}

fn part2(input: &str) -> impl Debug {
    let last = integers_unsigned::<usize>(input.lines().last().unwrap())[0];
    let (vals, moduli): (Vec<_>, Vec<_>) = input
        .lines()
        .chain(std::iter::once(format!("{} 11 0 0", last + 1).as_str()))
        .map(integers_signed::<isize>)
        .map(|v| (v[1], (v[1] - v[3] - v[0]).rem_euclid(v[1])))
        .unzip();
    crt(&vals, &moduli)
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/15")));
    println!("{:?}", part2(include_str!("../../input/15")));
}
