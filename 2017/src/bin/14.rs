#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

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

fn knot_hash(input: &str) -> u32 {
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
        .map(|c| c.iter().fold(0, |acc, e| acc ^ e).count_ones())
        .sum::<u32>()
}

fn part1(input: &str) -> impl Debug {
    (0..128)
        .map(|i| knot_hash(format!("{}-{i}", "flqrgnkx".trim()).as_str()))
        .sum::<u32>()
}

fn knot_hash_set(input: &str, row: isize) -> HashSet<(isize, isize)> {
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
        .enumerate()
        .flat_map(|(idx, v)| {
            (0..8)
                .filter(move |j| v & 0x1 << (7 - j) > 0)
                .map(move |j| (row, j + 8 * idx as isize))
        })
        .collect()
}

fn get_group(
    start: (isize, isize),
    set: &HashSet<(isize, isize)>,
) -> HashSet<(isize, isize)> {
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    q.push_back(start);

    while let Some(node) = q.pop_front() {
        if seen.contains(&node) {
            continue;
        }
        seen.insert(node);

        for d in DIRS {
            let new = add2(node, d);
            if set.contains(&new) {
                q.push_back(new);
            }
        }
    }

    seen
}

fn part2(input: &str) -> impl Debug {
    let set = (0..128)
        .flat_map(|i| {
            knot_hash_set(format!("{}-{i}", input.trim()).as_str(), i)
        })
        .collect::<HashSet<_>>();

    for i in 0..128 {
        let mut s = String::new();
        for j in 0..128 {
            s.push(if set.contains(&(i, j)) { '#' } else { '.' });
        }
    }

    let mut seen = HashSet::new();
    let mut groups = 0;
    for k in (0..128)
        .cartesian_product(0..128)
        .filter(|k| set.contains(k))
    {
        if seen.contains(&k) {
            continue;
        }

        if k == (0, 1) {
            println!("{:?}", get_group(k, &set));
        }
        seen.extend(get_group(k, &set));
        groups += 1;
    }

    groups
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/14")));
    println!("{:?}", part2(include_str!("../../input/14")));
}
