#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &str) -> impl Debug {
    let v = integers_unsigned::<usize>(input);
    let mut a = v[0];
    let mut b = v[1];

    (0..40_000_000)
        .filter(|_| {
            a *= 16807;
            a %= 2147483647;
            b *= 48271;
            b %= 2147483647;

            a & 0xffff == b & 0xffff
        })
        .count()
}

fn part2(input: &str) -> impl Debug {
    let v = integers_unsigned::<usize>(input);
    let mut a = v[0];
    let mut b = v[1];

    (0..5_000_000)
        .filter(|_| {
            loop {
                a *= 16807;
                a %= 2147483647;
                if a % 4 == 0 {
                    break;
                }
            }
            loop {
                b *= 48271;
                b %= 2147483647;
                if b % 8 == 0 {
                    break;
                }
            }

            a & 0xffff == b & 0xffff
        })
        .count()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/15")));
    println!("{:?}", part2(include_str!("../../input/15")));
}
