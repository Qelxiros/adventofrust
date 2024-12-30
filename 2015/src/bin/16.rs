#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn part1(input: &str) -> impl Debug {
    input
        .lines()
        .filter_map(|s| {
            s.split_once(": ")
                .map(|(a, b)| {
                    (
                        integers_unsigned::<usize>(a)[0],
                        b.split(", ")
                            .map(|i| {
                                i.split_once(": ")
                                    .map(|(a, b)| {
                                        (a, b.parse::<usize>().unwrap())
                                    })
                                    .unwrap()
                            })
                            .collect::<HashMap<_, _>>(),
                    )
                })
                .filter(|(_, map)| {
                    map.get("children").is_none_or(|&a| a == 3)
                        && map.get("cats").is_none_or(|&a| a == 7)
                        && map.get("samoyeds").is_none_or(|&a| a == 2)
                        && map.get("pomeranians").is_none_or(|&a| a == 3)
                        && map.get("akitas").is_none_or(|&a| a == 0)
                        && map.get("vizslas").is_none_or(|&a| a == 0)
                        && map.get("goldfish").is_none_or(|&a| a == 5)
                        && map.get("trees").is_none_or(|&a| a == 3)
                        && map.get("cars").is_none_or(|&a| a == 2)
                        && map.get("perfumes").is_none_or(|&a| a == 1)
                })
        })
        .collect_vec()[0]
        .0
}

fn part2(input: &str) -> impl Debug {
    input
        .lines()
        .filter_map(|s| {
            s.split_once(": ")
                .map(|(a, b)| {
                    (
                        integers_unsigned::<usize>(a)[0],
                        b.split(", ")
                            .map(|i| {
                                i.split_once(": ")
                                    .map(|(a, b)| {
                                        (a, b.parse::<usize>().unwrap())
                                    })
                                    .unwrap()
                            })
                            .collect::<HashMap<_, _>>(),
                    )
                })
                .filter(|(_, map)| {
                    map.get("children").is_none_or(|&a| a == 3)
                        && map.get("cats").is_none_or(|&a| a > 7)
                        && map.get("samoyeds").is_none_or(|&a| a == 2)
                        && map.get("pomeranians").is_none_or(|&a| a < 3)
                        && map.get("akitas").is_none_or(|&a| a == 0)
                        && map.get("vizslas").is_none_or(|&a| a == 0)
                        && map.get("goldfish").is_none_or(|&a| a < 5)
                        && map.get("trees").is_none_or(|&a| a > 3)
                        && map.get("cars").is_none_or(|&a| a == 2)
                        && map.get("perfumes").is_none_or(|&a| a == 1)
                })
        })
        .collect_vec()[0]
        .0
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/16")));
    println!("{:?}", part2(include_str!("../../input/16")));
}
