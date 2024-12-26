#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use itertools::Itertools;

fn next(mut num: isize) -> isize {
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
        .map(|s| s.parse::<isize>().unwrap())
        .map(|mut n| {
            for _ in 0..2000 {
                n = next(n);
            }
            n
        })
        .sum::<isize>()
}

fn part2(input: &str) -> impl Debug {
    let mut map = HashMap::new();

    input
        .lines()
        .map(|s| s.parse::<isize>().unwrap())
        .map(|n| {
            let mut v = vec![n % 10];
            let mut cur = n;
            for _ in 0..2000 {
                cur = next(cur);
                v.push(cur % 10);
            }

            v
        })
        .for_each(|v| {
            let mut seen = HashSet::new();
            v.into_iter()
                .tuple_windows::<(_, _, _, _, _)>()
                .for_each(|a| {
                    let diff = (a.1 - a.0, a.2 - a.1, a.3 - a.2, a.4 - a.3);
                    if !seen.contains(&diff) {
                        *map.entry(diff).or_insert(0) += a.4;
                        seen.insert(diff);
                    }
                })
        });

    map.values().max().copied().unwrap()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/22")));
    println!("{:?}", part2(include_str!("../../input/22")));
}
