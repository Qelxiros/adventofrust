#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn part1(input: &str) -> impl Debug {
    let packages = integers_unsigned::<usize>(input);
    let target = packages.iter().sum::<usize>() / 3;
    let mut best = (usize::MAX, usize::MAX);
    for i in 0..packages.len() {
        let p1s = packages
            .iter()
            .copied()
            .combinations(i)
            .filter(|v| v.iter().sum::<usize>() == target)
            .collect_vec();
        for (idx, p1) in p1s.into_iter().enumerate() {
            'inner: for j in 0..(packages.len() - i) {
                if packages
                    .iter()
                    .copied()
                    .filter(|a| !p1.contains(a))
                    .combinations(j)
                    .any(|v| v.iter().sum::<usize>() == target)
                {
                    let len = p1.len();
                    let qe = p1.iter().product::<usize>();
                    best = best.min((len, qe));
                    break 'inner;
                }
            }
        }
        if best.0 < usize::MAX {
            break;
        }
    }

    best.1
}

fn part2(input: &str) -> impl Debug {
    let packages = integers_unsigned::<usize>(input);
    let target = packages.iter().sum::<usize>() / 4;
    let mut best = (usize::MAX, usize::MAX);
    for i in 0..packages.len() {
        let p1s = packages
            .iter()
            .copied()
            .combinations(i)
            .filter(|v| v.iter().sum::<usize>() == target)
            .collect_vec();
        for (idx, p1) in p1s.into_iter().enumerate() {
            if (0..(packages.len() - i)).any(|j| {
                packages
                    .iter()
                    .copied()
                    .filter(|a| !p1.contains(a))
                    .combinations(j)
                    .any(|v| {
                        v.iter().sum::<usize>() == target
                            && (0..(packages.len() - i - j)).any(|k| {
                                packages
                                    .iter()
                                    .copied()
                                    .filter(|a| {
                                        !p1.contains(a) && !v.contains(a)
                                    })
                                    .combinations(k)
                                    .any(|v| v.iter().sum::<usize>() == target)
                            })
                    })
            }) {
                let len = p1.len();
                let qe = p1.iter().product::<usize>();
                best = best.min((len, qe));
            }
        }
        if best.0 < usize::MAX {
            break;
        }
    }

    best.1
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/24")));
    println!("{:?}", part2(include_str!("../../input/24")));
}
