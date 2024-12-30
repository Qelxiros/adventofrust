#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn next(mut pass: Vec<char>, alpha: &[char]) -> Vec<char> {
    let mut idx = pass.len() - 1;
    loop {
        let i = (alpha.iter().position(|c| *c == pass[idx]).unwrap() + 1) % 26;
        pass[idx] = alpha[i];
        if i != 0 {
            break;
        }
        idx -= 1;
    }

    pass
}

fn check(pass: &[char]) -> bool {
    pass.iter().tuple_windows().any(|(&a, &b, &c)| {
        a as u32 + 1 == b as u32 && b as u32 + 1 == c as u32
    }) && pass.iter().all(|c| !"ilo".contains(*c))
        && pass
            .iter()
            .tuple_windows()
            .enumerate()
            .filter(|(_, (a, b))| a == b)
            .map(|(idx, _)| idx)
            .tuple_windows()
            .any(|(a, b)| b - a > 1)
}

fn part1(input: &str) -> String {
    let alpha = "abcdefghijklmnopqrstuvwxyz".chars().collect_vec();
    let mut input = input.trim().chars().collect_vec();
    loop {
        input = next(input, &alpha);
        if check(&input) {
            return input.into_iter().collect::<String>();
        }
    }
}

fn part2(input: &str) -> impl Debug {
    let alpha = "abcdefghijklmnopqrstuvwxyz".chars().collect_vec();
    let mut input = part1(input).trim().chars().collect_vec();
    loop {
        input = next(input, &alpha);
        if check(&input) {
            return input.into_iter().collect::<String>();
        }
    }
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/11")));
    println!("{:?}", part2(include_str!("../../input/11")));
}
