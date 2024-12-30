#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn part1(input: &str) -> impl Debug {
    let ingredients = input
        .lines()
        .filter_map(|s| s.split_once(':'))
        .map(|(_, props)| integers_signed::<isize>(props))
        .collect_vec();

    ingredients
        .into_iter()
        .combinations_with_replacement(100)
        .map(|v| {
            v.clone()
                .into_iter()
                .map(|v1| (v1[0], v1[1], v1[2], v1[3]))
                .fold(vec![0, 0, 0, 0], |acc, v| {
                    vec![acc[0] + v.0, acc[1] + v.1, acc[2] + v.2, acc[3] + v.3]
                })
                .into_iter()
                .map(|a| if a < 0 { 0 } else { a })
                .product::<isize>()
        })
        .max()
        .unwrap()
}

fn part2(input: &str) -> impl Debug {
    let ingredients3 = input
        .lines()
        .take(2)
        .filter_map(|s| s.split_once(':'))
        .map(|(_, props)| integers_signed::<isize>(props))
        .collect_vec();
    let ingredients8 = input
        .lines()
        .skip(2)
        .filter_map(|s| s.split_once(':'))
        .map(|(_, props)| integers_signed::<isize>(props))
        .collect_vec();

    ingredients3
        .into_iter()
        .combinations_with_replacement(60)
        .cartesian_product(
            ingredients8.into_iter().combinations_with_replacement(40),
        )
        .map(|(a, b)| {
            a.into_iter()
                .chain(b)
                .map(|v1| (v1[0], v1[1], v1[2], v1[3]))
                .fold(vec![0, 0, 0, 0], |acc, v| {
                    vec![acc[0] + v.0, acc[1] + v.1, acc[2] + v.2, acc[3] + v.3]
                })
                .into_iter()
                .map(|a| if a < 0 { 0 } else { a })
                .product::<isize>()
        })
        .max()
        .unwrap()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/15")));
    println!("{:?}", part2(include_str!("../../input/15")));
}
