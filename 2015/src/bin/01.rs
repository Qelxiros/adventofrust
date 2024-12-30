#![feature(let_chains)]

use std::fmt::Debug;

fn part1(input: &str) -> impl Debug {
    input
        .lines()
        .flat_map(|s| s.chars().map(|c| if c == '(' { 1 } else { -1 }))
        .sum::<isize>()
}

fn part2(input: &str) -> impl Debug {
    let mut total = 0;
    input
        .lines()
        .flat_map(|s| s.chars())
        .position(|c| {
            total += if c == '(' { 1 } else { -1 };
            total == -1
        })
        .unwrap()
        + 1
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/01")));
    println!("{:?}", part2(include_str!("../../input/01")));
}
