#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn fight(
    mut player: (usize, usize, usize),
    mut boss: (usize, usize, usize),
) -> bool {
    loop {
        boss.0 = boss
            .0
            .saturating_sub(player.1.saturating_sub(boss.2).max(1));
        if boss.0 == 0 {
            return true;
        }
        player.0 = player
            .0
            .saturating_sub(boss.1.saturating_sub(player.2).max(1));
        if player.0 == 0 {
            return false;
        }
    }
}

const SHOP: [&[(usize, usize, usize)]; 4] = [
    &[(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)],
    &[
        (13, 0, 1),
        (31, 0, 2),
        (53, 0, 3),
        (75, 0, 4),
        (102, 0, 5),
        (0, 0, 0),
    ],
    &[
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
        (0, 0, 0),
    ],
    &[
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
        (0, 0, 0),
    ],
];

fn part1(input: &str) -> impl Debug {
    let input = integers_unsigned(input);
    let boss = (input[0], input[1], input[2]);

    SHOP.into_iter()
        .multi_cartesian_product()
        .filter(|v| v[2] != v[3])
        .map(|v| v.into_iter().copied().fold((0, 0, 0), add3))
        .filter(move |(_, damage, armor)| fight((100, *damage, *armor), boss))
        .map(|(cost, _, _)| cost)
        .min()
        .unwrap()
}

fn part2(input: &str) -> impl Debug {
    let input = integers_unsigned(input);
    let boss = (input[0], input[1], input[2]);

    SHOP.into_iter()
        .multi_cartesian_product()
        .filter(|v| v[2] != v[3])
        .map(|v| v.into_iter().copied().fold((0, 0, 0), add3))
        .filter(move |(_, damage, armor)| !fight((100, *damage, *armor), boss))
        .map(|(cost, _, _)| cost)
        .max()
        .unwrap()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/21")));
    println!("{:?}", part2(include_str!("../../input/21")));
}
