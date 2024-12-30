#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use num::traits::Euclid;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &str) -> impl Debug {
    let input = integers_unsigned::<usize>(input)[0];
    let mut elves = (1..=input).collect::<VecDeque<_>>();

    while elves.len() > 1 {
        let wrap = elves.len() % 2 == 1;
        elves = elves.into_iter().step_by(2).collect::<VecDeque<_>>();
        if wrap {
            elves.pop_front();
        }
    }

    elves.pop_front().unwrap()
}

// pattern: count by ones to the previous power of three, then by twos to the
// current power of three, then repeat for the next power of three
struct Part2Solver {
    exp: u32,
    progress: usize,
}

impl Part2Solver {
    fn new() -> Self {
        Self {
            exp: 1,
            progress: 0,
        }
    }
}

impl Iterator for Part2Solver {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.progress += if self.progress >= 3usize.pow(self.exp - 1) {
            2
        } else {
            1
        };
        let out = self.progress;
        if self.progress == 3usize.pow(self.exp) {
            self.exp += 1;
            self.progress = 0;
        }
        Some(out)
    }
}

fn part2(input: &str) -> impl Debug {
    let input = integers_unsigned::<usize>(input)[0];

    let mut solver = Part2Solver::new();

    solver.nth(input - 2).unwrap()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/19")));
    println!("{:?}", part2(include_str!("../../input/19")));
}
