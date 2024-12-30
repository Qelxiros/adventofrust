#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn part1(input: &str) -> impl Debug {
    let mut set = HashSet::new();
    set.insert((0, 0));
    let mut loc = (0, 0);
    input.lines().flat_map(|s| s.chars()).for_each(|c| {
        let delta = match c {
            '^' => DIRS[0],
            '>' => DIRS[1],
            'v' => DIRS[2],
            '<' => DIRS[3],
            _ => panic!(),
        };

        loc = (loc.0 + delta.0, loc.1 + delta.1);
        set.insert(loc);
    });

    set.len()
}

fn part2(input: &str) -> impl Debug {
    let mut set = HashSet::new();
    set.insert((0, 0));
    let mut loc1 = (0, 0);
    let mut loc2 = (0, 0);
    input
        .lines()
        .flat_map(|s| s.chars())
        .enumerate()
        .for_each(|(idx, c)| {
            let delta = match c {
                '^' => DIRS[0],
                '>' => DIRS[1],
                'v' => DIRS[2],
                '<' => DIRS[3],
                _ => panic!(),
            };

            if idx % 2 == 0 {
                loc1 = (loc1.0 + delta.0, loc1.1 + delta.1);
                set.insert(loc1);
            } else {
                loc2 = (loc2.0 + delta.0, loc2.1 + delta.1);
                set.insert(loc2);
            }
        });

    set.len()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/03")));
    println!("{:?}", part2(include_str!("../../input/03")));
}
