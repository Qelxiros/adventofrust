#![feature(let_chains)]

use std::{collections::*, fmt::Debug, iter::once};

use itertools::{Itertools, repeat_n};
use regex::Regex;

use adventofrust::utils::*;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn next(mut num: usize) -> usize {
    num ^= num * 64;
    num %= 16777216;
    num ^= num / 32;
    num %= 16777216;
    num ^= num * 2048;
    num %= 16777216;

    num
}

fn part1(input: &str) -> impl Debug {
    input
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .map(|mut n| {
            for _ in 0..2000 {
                n = next(n);
            }
            n
        })
        .sum::<usize>()
}

fn score(seq: (isize, isize, isize, isize), seqs: &[Vec<usize>]) -> usize {
    seqs.iter()
        .map(|v| {
            v.into_iter()
                .map(|&a| a as isize)
                .tuple_windows::<(_, _, _, _, _)>()
                .position(|(a, b, c, d, e)| (b - a, c - b, d - c, e - d) == seq)
                .map(|i| v[i + 4])
                .unwrap_or(0)
        })
        .sum()
}

fn part2(input: &str) -> impl Debug {
    let seqs = input
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .map(|n| {
            let mut v = vec![n % 10];
            let mut cur = n;
            for _ in 0..2000 {
                cur = next(cur);
                v.push(cur % 10);
            }

            v
        })
        .collect_vec();

    repeat_n(-9..10, 4)
        .multi_cartesian_product()
        .map(|a| score((a[0], a[1], a[2], a[3]), &seqs))
        .max()
        .unwrap()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/22")));
    println!("{:?}", part2(include_str!("../../input/22")));
}
