#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn process(speed: usize, duration: usize, rest: usize, total: usize) -> usize {
    ((total / (duration + rest)) * speed * duration)
        + ((total % (duration + rest)).min(duration) * speed)
}

fn part1(input: &str) -> impl Debug {
    input
        .lines()
        .map(integers_unsigned)
        .map(|v| process(v[0], v[1], v[2], 2503))
        .max()
        .unwrap()
}

#[derive(Debug, Clone, Copy)]
struct Reindeer {
    pos: usize,
    speed: usize,
    dur: usize,
    dur_used: usize,
    rest: usize,
    rest_rem: usize,
    score: usize,
}

fn part2(input: &str) -> impl Debug {
    let mut r = input
        .lines()
        .map(integers_unsigned)
        .map(|v| Reindeer {
            pos: 0,
            speed: v[0],
            dur: v[1],
            dur_used: 0,
            rest: v[2],
            rest_rem: 0,
            score: 0,
        })
        .collect_vec();

    for _ in 0..2503 {
        for &mut Reindeer {
            ref mut pos,
            speed,
            dur,
            ref mut dur_used,
            rest,
            ref mut rest_rem,
            score,
        } in r.iter_mut()
        {
            if *dur_used < dur {
                *pos += speed;
                *dur_used += 1;
                if *dur_used == dur {
                    *rest_rem = rest;
                }
            } else if *rest_rem > 0 {
                *rest_rem -= 1;
                if *rest_rem == 0 {
                    *dur_used = 0;
                }
            }
        }

        let temp = r.clone();
        let leaders = temp
            .into_iter()
            .enumerate()
            .max_set_by_key(|(_, reindeer)| reindeer.pos);
        let leaders = leaders.into_iter().map(|(idx, _)| idx);
        for idx in leaders {
            r[idx].score += 1;
        }
    }

    r.into_iter().map(|reindeer| reindeer.score).max().unwrap()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/14")));
    println!("{:?}", part2(include_str!("../../input/14")));
}
