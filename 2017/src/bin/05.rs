#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &str) -> impl Debug {
    let mut v = integers_signed::<isize>(input);

    let mut steps = 0;
    let mut idx = 0;
    loop {
        steps += 1;
        let jump = v[idx];
        v[idx] += 1;
        idx = (idx as isize + jump) as usize;
        if idx >= v.len() {
            return steps;
        }
    }
}

fn part2(input: &str) -> impl Debug {
    let mut v = integers_signed::<isize>(input);

    let mut steps = 0;
    let mut idx = 0;
    loop {
        steps += 1;
        let jump = v[idx];
        v[idx] += if jump >= 3 { -1 } else { 1 };
        idx = (idx as isize + jump) as usize;
        if idx >= v.len() {
            return steps;
        }
    }
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/05")));
    println!("{:?}", part2(include_str!("../../input/05")));
}
