#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use num::PrimInt;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn is_valid(pos: (usize, usize), f: usize) -> bool {
    (pos.0 * pos.0 + 3 * pos.0 + 2 * pos.0 * pos.1 + pos.1 + pos.1 * pos.1 + f)
        .count_ones()
        % 2
        == 0
}

fn part1(input: &str) -> impl Debug {
    let favorite_num = integers_unsigned(input)[0];
    let target = (31, 39);

    let mut q = VecDeque::new();
    q.push_back(((1, 1), 0));
    let mut seen = HashSet::new();

    while let Some((pos, steps)) = q.pop_front() {
        if seen.contains(&pos) {
            continue;
        }
        seen.insert(pos);
        if pos == target {
            return steps;
        }
        q.extend(
            DIRS.into_iter()
                .map(|dir| {
                    (
                        (
                            (pos.0 as isize + dir.0) as usize,
                            (pos.1 as isize + dir.1) as usize,
                        ),
                        steps + 1,
                    )
                })
                .filter(|p| is_valid(p.0, favorite_num)),
        );
    }

    0
}

fn part2(input: &str) -> impl Debug {
    let favorite_num = integers_unsigned(input)[0];

    let mut q = VecDeque::new();
    q.push_back(((1, 1), 0));
    let mut seen = HashSet::new();

    while let Some((pos, steps)) = q.pop_front() {
        if seen.contains(&pos) || steps > 50 {
            continue;
        }
        seen.insert(pos);
        q.extend(
            DIRS.into_iter()
                .filter(|dir| {
                    pos.0 as isize + dir.0 >= 0 && pos.1 as isize + dir.1 >= 0
                })
                .map(|dir| {
                    (
                        (
                            (pos.0 as isize + dir.0) as usize,
                            (pos.1 as isize + dir.1) as usize,
                        ),
                        steps + 1,
                    )
                })
                .filter(|p| is_valid(p.0, favorite_num)),
        );
    }

    seen.len()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/13")));
    println!("{:?}", part2(include_str!("../../input/13")));
}
