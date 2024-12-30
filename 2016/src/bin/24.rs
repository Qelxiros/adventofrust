#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &str) -> impl Debug {
    let (grid, locations) = grid_idx_with(
        input,
        "01234567".chars().collect_vec().as_slice(),
        |c| c != '#',
    );

    let start = (locations[0].unwrap(), Vec::new(), 0);
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    q.push_back(start);

    let locations = locations
        .into_iter()
        .map(Option::unwrap)
        .enumerate()
        .skip(1)
        .map(|(a, b)| (b, a))
        .collect::<HashMap<_, _>>();

    while let Some((pos, visited, steps)) = q.pop_front() {
        if seen.contains(&(pos, visited.iter().join(""))) {
            continue;
        }
        seen.insert((pos, visited.iter().join("")));
        if visited.len() == 7 {
            return steps;
        }

        for d in DIRS {
            let new = (
                ((pos.0 as isize + d.0) as usize),
                (pos.1 as isize + d.1) as usize,
            );
            if grid[new.0][new.1] {
                let mut visited = visited.clone();
                if let Some(x) = locations.get(&new) {
                    let Err(i) = visited.binary_search(&x) else {
                        continue;
                    };
                    visited.insert(i, x);
                }
                q.push_back((new, visited, steps + 1));
            }
        }
    }

    0
}

fn part2(input: &str) -> impl Debug {
    let (grid, locations) = grid_idx_with(
        input,
        "01234567".chars().collect_vec().as_slice(),
        |c| c != '#',
    );

    let start = (locations[0].unwrap(), Vec::new(), 0);
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    q.push_back(start);

    let locations = locations
        .into_iter()
        .map(Option::unwrap)
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect::<HashMap<_, _>>();

    while let Some((pos, visited, steps)) = q.pop_front() {
        if seen.contains(&(pos, visited.iter().join(""))) {
            continue;
        }
        seen.insert((pos, visited.iter().join("")));
        if visited.len() == 8 {
            return steps;
        }

        for d in DIRS {
            let new = (
                ((pos.0 as isize + d.0) as usize),
                (pos.1 as isize + d.1) as usize,
            );
            if grid[new.0][new.1] {
                let mut visited = visited.clone();
                if let Some(x) = locations.get(&new) {
                    let Err(i) = visited.binary_search(&x) else {
                        continue;
                    };
                    if *x == 0 && visited.len() < 7 {
                        continue;
                    }
                    visited.insert(i, x);
                }
                q.push_back((new, visited, steps + 1));
            }
        }
    }

    println!(
        "{:?}",
        seen.into_iter().filter(|(_, v)| v.len() == 7).collect_vec()
    );

    0
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/24")));
    println!("{:?}", part2(include_str!("../../input/24")));
}
