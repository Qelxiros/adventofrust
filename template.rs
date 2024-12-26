#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use itertools::Itertools;
use regex::Regex;

use common::utils::*;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn part1(input: &str) -> impl Debug {}

fn part2(input: &str) -> impl Debug {}

fn main() {
    println!("{:?}", part1(include_str!("../../input/%DAY%")));
    println!("{:?}", part2(include_str!("../../input/%DAY%")));
}
