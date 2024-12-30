#![feature(let_chains)]

use std::fmt::Debug;

use common::utils::*;

fn part1(input: &str) -> impl Debug {
    input
        .lines()
        .map(integers_unsigned::<usize>)
        .map(|v| {
            let a = v[0] * v[1];
            let b = v[1] * v[2];
            let c = v[2] * v[0];
            2 * a + 2 * b + 2 * c + a.min(b).min(c)
        })
        .sum::<usize>()
}

fn part2(input: &str) -> impl Debug {
    input
        .lines()
        .map(integers_unsigned::<usize>)
        .map(|v| {
            let a = v[0] * v[1];
            let b = v[1] * v[2];
            let c = v[2] * v[0];
            v[0] * v[1] * v[2] + a.min(b).min(c)
        })
        .sum::<usize>()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/02")));
    println!("{:?}", part2(include_str!("../../input/02")));
}
