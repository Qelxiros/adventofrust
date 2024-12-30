#![feature(let_chains)]

use std::{cmp::Ordering, collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &str) -> impl Debug {
    let components = input
        .lines()
        .map(integers_unsigned::<usize>)
        .map(|v| (v[0], v[1]))
        .collect_vec();

    let mut q = components
        .iter()
        .filter(|(a, b)| *a == 0 || *b == 0)
        .map(|a| vec![a])
        .collect::<VecDeque<_>>();

    let mut out = 0;
    while let Some(v) = q.pop_front() {
        out = out.max(v.iter().map(|(a, b)| a + b).sum::<usize>());

        let last = v.last().unwrap();
        let free = if v.len() == 1 {
            last.0.max(last.1)
        } else {
            let p = v[v.len() - 2];
            if last.0 == p.0 || last.0 == p.1 {
                last.1
            } else {
                last.0
            }
        };

        for c in &components {
            if v.contains(&c) || c.0 != free && c.1 != free {
                continue;
            }

            let mut new = v.clone();
            new.push(c);
            q.push_back(new);
        }
    }

    out
}

fn part2(input: &str) -> impl Debug {
    let components = input
        .lines()
        .map(integers_unsigned::<usize>)
        .map(|v| (v[0], v[1]))
        .collect_vec();

    let mut q = components
        .iter()
        .filter(|(a, b)| *a == 0 || *b == 0)
        .map(|a| vec![a])
        .collect::<VecDeque<_>>();

    let mut max_len = 0;
    let mut max_str = 0;
    while let Some(v) = q.pop_front() {
        match v.len().cmp(&max_len) {
            Ordering::Less => {}
            Ordering::Equal => {
                max_str =
                    max_str.max(v.iter().map(|(a, b)| a + b).sum::<usize>());
            }
            Ordering::Greater => {
                max_len = v.len();
                max_str = v.iter().map(|(a, b)| a + b).sum::<usize>();
            }
        }

        let last = v.last().unwrap();
        let free = if v.len() == 1 {
            last.0.max(last.1)
        } else {
            let p = v[v.len() - 2];
            if last.0 == p.0 || last.0 == p.1 {
                last.1
            } else {
                last.0
            }
        };

        for c in &components {
            if v.contains(&c) || c.0 != free && c.1 != free {
                continue;
            }

            let mut new = v.clone();
            new.push(c);
            q.push_back(new);
        }
    }

    max_str
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/24")));
    println!("{:?}", part2(include_str!("../../input/24")));
}
