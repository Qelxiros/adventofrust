#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &str) -> impl Debug {
    let nodes = input
        .lines()
        .skip(2)
        .map(integers_unsigned::<usize>)
        .map(|v| ((v[0], v[1]), v[3], v[4]))
        .collect_vec();

    nodes
        .iter()
        .cartesian_product(&nodes)
        .filter(|((a, _, _), (b, _, _))| a != b)
        .filter(|((_, used, _), (_, _, avail))| avail >= used && *used > 0)
        .count()
}

fn part2(input: &str) -> impl Debug {
    let mut empty_pos = (0isize, 0isize);
    let mut max_x = 0;
    let nodes = input
        .lines()
        .skip(2)
        .map(integers_unsigned::<usize>)
        .map(|v| {
            max_x = max_x.max(v[0]);
            ((v[0], v[1]), v[3], v[4])
        })
        .map(|(pos, used, avail)| {
            (
                (pos.0 as isize, pos.1 as isize),
                if used == 0 {
                    empty_pos = (pos.0 as isize, pos.1 as isize);
                    '.'
                } else if used + avail < 100 {
                    '.'
                } else {
                    '#'
                },
            )
        })
        .collect::<HashMap<_, _>>();

    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    q.push_back((empty_pos, (max_x as isize, 0), 0));

    while let Some((empty_pos, data_pos, steps)) = q.pop_front() {
        if seen.contains(&(empty_pos, data_pos)) {
            continue;
        }
        seen.insert((empty_pos, data_pos));

        if data_pos == (0, 0) {
            return steps;
        }

        for d in DIRS {
            let new = add2(d, empty_pos);
            if let Some(c) = nodes.get(&new)
                && *c == '.'
            {
                q.push_back((
                    new,
                    if new == data_pos { empty_pos } else { data_pos },
                    steps + 1,
                ));
            }
        }
    }

    0
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/22")));
    println!("{:?}", part2(include_str!("../../input/22")));
}
