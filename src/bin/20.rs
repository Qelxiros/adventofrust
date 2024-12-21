#![feature(let_chains)]

use std::{
    collections::*,
    fmt::{Debug, Display},
};

use itertools::Itertools;
use regex::Regex;

use adventofrust::utils::*;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn part1(input: &str) -> impl Display + Debug {
    let (grid, indices) = grid_idx(input, ['S', 'E']);
    let indices = indices
        .unwrap()
        .into_iter()
        .map(|(a, b)| (a as isize, b as isize))
        .collect_vec();

    let mut order = HashMap::new();
    order.insert(indices[0], 0);
    let mut cur = indices[0];
    'outer: loop {
        if cur == indices[1] {
            break;
        }
        for d in DIRS {
            if grid[(cur.0 + d.0) as usize][(cur.1 + d.1) as usize] != '#'
                && !order.contains_key(&(cur.0 + d.0, cur.1 + d.1))
            {
                order.insert((cur.0 + d.0, cur.1 + d.1), order[&cur] + 1);
                cur = (cur.0 + d.0, cur.1 + d.1);
                continue 'outer;
            }
        }
    }

    let mut count = 0;

    for (pos, idx) in order.clone().into_iter() {
        for d1 in 0..4 {
            for d2 in d1..4 {
                let d1 = DIRS[d1];
                let d2 = DIRS[d2];
                let d = (d1.0 + d2.0, d1.1 + d2.1);
                if d == (0, 0) {
                    continue;
                }
                if let Some(i) = order.get(&(pos.0 + d.0, pos.1 + d.1))
                    && i - idx - 2 >= 100
                {
                    count += 1;
                }
            }
        }
    }

    count
}

fn cheat_neighbors() -> impl Iterator<Item = (isize, isize)> {
    (0..=20)
        .flat_map(|r| {
            (0..=r).flat_map(move |a| vec![(a, r - a), (a, a - r), (-a, r - a), (-a, a - r)])
        })
        .unique()
}

fn part2(input: &str) -> impl Display {
    let (grid, indices) = grid_idx(input, ['S', 'E']);
    let indices = indices
        .unwrap()
        .into_iter()
        .map(|(a, b)| (a as isize, b as isize))
        .collect_vec();

    let mut order = HashMap::new();
    order.insert(indices[0], 0);
    let mut cur = indices[0];
    'outer: loop {
        if cur == indices[1] {
            break;
        }
        for d in DIRS {
            if grid[(cur.0 + d.0) as usize][(cur.1 + d.1) as usize] != '#'
                && !order.contains_key(&(cur.0 + d.0, cur.1 + d.1))
            {
                order.insert((cur.0 + d.0, cur.1 + d.1), order[&cur] + 1);
                cur = (cur.0 + d.0, cur.1 + d.1);
                continue 'outer;
            }
        }
    }

    let mut cheats = HashSet::new();

    for (pos, idx) in order.clone().into_iter() {
        for d in cheat_neighbors() {
            let cheat_len = d.0.abs() + d.1.abs();
            if let Some(i) = order.get(&(pos.0 + d.0, pos.1 + d.1))
                && i - idx - cheat_len >= 100
            {
                cheats.insert((pos, (pos.0 + d.0, pos.1 + d.1)));
            }
        }
    }

    cheats.len()
}

fn main() {
    println!("{}", part1(include_str!("../../input/20")));
    println!("{}", part2(include_str!("../../input/20")));
}
