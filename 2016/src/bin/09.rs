#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn decompress(input: &str, re: Regex) -> String {
    let Some((start, end, mat)) = re
        .find(input)
        .map(|mat| (mat.start(), mat.end(), mat.as_str()))
    else {
        return input.to_string();
    };
    let v = integers_unsigned::<usize>(mat);
    let decompressed = std::iter::once(&input[end..end + v[0]])
        .cycle()
        .take(v[1])
        .collect::<String>();
    format!(
        "{}{}{}",
        &input[0..start],
        decompressed,
        decompress(&input[end + v[0]..], re)
    )
}

fn part1(input: &str) -> impl Debug {
    let re = Regex::new(r"\(\d+x\d+\)").unwrap();
    decompress(input.trim(), re).len()
}

fn decompress2(input: &str, re: &Regex) -> usize {
    let Some((start, end, mat)) = re
        .find(input)
        .map(|mat| (mat.start(), mat.end(), mat.as_str()))
    else {
        return input.len();
    };
    let v = integers_unsigned::<usize>(mat);

    start
        + decompress2(&input[end..end + v[0]], re) * v[1]
        + decompress2(&input[end + v[0]..], re)
}

fn part2(input: &str) -> impl Debug {
    let re = Regex::new(r"\(\d+x\d+\)").unwrap();
    decompress2(input.trim(), &re)
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/09")));
    println!("{:?}", part2(include_str!("../../input/09")));
}
