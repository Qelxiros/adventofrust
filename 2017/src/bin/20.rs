#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &str) -> impl Debug {
    input
        .lines()
        .map(integers_signed::<isize>)
        .enumerate()
        .min_set_by_key(|(_, v)| v[6..9].iter().map(|i| i.abs()).sum::<isize>())
        .into_iter()
        .min_set_by_key(|(_, v)| v[3..6].iter().map(|i| i.abs()).sum::<isize>())
        .into_iter()
        .min_by_key(|(_, v)| (v[0..3].iter().map(|i| i.abs()).sum::<isize>()))
        .map(|(i, _)| i)
        .unwrap()
}

fn check(particles: &[Vec<isize>]) -> bool {
    particles
        .is_sorted_by_key(|v| v[0..3].iter().map(|i| i.abs()).sum::<isize>())
}

fn part2(input: &str) -> impl Debug {
    let mut v = input.lines().map(integers_signed::<isize>).collect_vec();

    while {
        v.sort_by_key(|v| {
            (
                v[6..9].iter().map(|i| i.abs()).sum::<isize>(),
                v[3..6].iter().map(|i| i.abs()).sum::<isize>(),
                v[0..3].iter().map(|i| i.abs()).sum::<isize>(),
            )
        });
        !check(&v)
    } {
        v.iter_mut().for_each(|v| {
            v[3] += v[6];
            v[4] += v[7];
            v[5] += v[8];
            v[0] += v[3];
            v[1] += v[4];
            v[2] += v[5];
        });

        let mut set = HashSet::new();
        for i in 0..v.len() {
            for j in (i + 1)..v.len() {
                if v[i][0..3] == v[j][0..3] {
                    set.insert(i);
                    set.insert(j);
                }
            }
        }
        v = v
            .into_iter()
            .enumerate()
            .filter(|(i, _)| !set.contains(i))
            .map(|(_, v)| v)
            .collect_vec();
    }

    v.len()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/20")));
    println!("{:?}", part2(include_str!("../../input/20")));
}
