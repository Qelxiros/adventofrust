#![feature(let_chains)]

use std::{
    collections::*,
    fmt::{Debug, Write},
};

use common::utils::*;
use itertools::{Itertools, repeat_n};
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn rev(s: &mut [usize], start: usize, end: usize) {
    s.rotate_left(start);
    let end = (end - start).rem_euclid(s.len());
    s[..end].reverse();
    s.rotate_right(start);
}

fn part1(input: &str) -> impl Debug {
    let len = 256;
    let mut list = (0..len).collect_vec();
    let mut pos = 0usize;
    let mut skip = 0;

    integers_unsigned::<usize>(input).into_iter().for_each(|i| {
        rev(&mut list, pos.rem_euclid(len), (pos + i).rem_euclid(len));
        pos += i + skip;
        skip += 1;
    });

    list[0] * list[1]
}

fn part2(input: &str) -> impl Debug {
    let len = 256;
    let mut list = (0..len).collect_vec();
    let mut pos = 0usize;
    let mut skip = 0;

    repeat_n(
        input
            .trim()
            .chars()
            .map(|c| c as usize)
            .chain([17, 31, 73, 47, 23]),
        64,
    )
    .flatten()
    .for_each(|i| {
        rev(&mut list, pos.rem_euclid(len), (pos + i).rem_euclid(len));
        pos += i + skip;
        skip += 1;
    });

    list.chunks(16)
        .map(|c| c.iter().fold(0, |acc, e| acc ^ e))
        .fold(String::new(), |mut acc, e| {
            write!(&mut acc, "{e:0>2x}").unwrap();
            acc
        })
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/10")));
    println!("{:?}", part2(include_str!("../../input/10")));
}
