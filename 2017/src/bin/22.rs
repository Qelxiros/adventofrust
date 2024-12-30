#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn burst(
    pos: (isize, isize),
    mut dir: usize,
    map: &mut HashMap<(isize, isize), bool>,
) -> ((isize, isize), usize, bool) {
    match map.entry(pos).or_insert(false) {
        true => {
            dir += 1;
            dir %= 4;
            map.entry(pos).and_modify(|b| *b = false);
            (add2(pos, DIRS[dir]), dir, false)
        }
        false => {
            dir += 3;
            dir %= 4;
            map.entry(pos).and_modify(|b| *b = true);
            (add2(pos, DIRS[dir]), dir, true)
        }
    }
}

fn part1(input: &str) -> impl Debug {
    let mut map = input
        .lines()
        .enumerate()
        .flat_map(|(i, s)| {
            s.chars()
                .enumerate()
                .map(move |(j, c)| ((i as isize, j as isize), c == '#'))
        })
        .collect::<HashMap<_, _>>();

    let mut pos = (
        (input.lines().count() / 2) as isize,
        (input.lines().next().unwrap().chars().count() / 2) as isize,
    );
    let mut dir = 0;
    let mut out = 0;

    for _ in 0..10_000 {
        let res = burst(pos, dir, &mut map);
        pos = res.0;
        dir = res.1;
        if res.2 {
            out += 1
        }
    }

    out
}

#[repr(isize)]
enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

fn burst2(
    pos: (isize, isize),
    mut dir: usize,
    map: &mut HashMap<(isize, isize), State>,
) -> ((isize, isize), usize, bool) {
    match map.entry(pos).or_insert(State::Clean) {
        State::Clean => {
            dir += 3;
            dir %= 4;
            map.entry(pos).and_modify(|b| *b = State::Weakened);
            (add2(pos, DIRS[dir]), dir, false)
        }
        State::Weakened => {
            map.entry(pos).and_modify(|b| *b = State::Infected);
            (add2(pos, DIRS[dir]), dir, true)
        }
        State::Infected => {
            dir += 1;
            dir %= 4;
            map.entry(pos).and_modify(|b| *b = State::Flagged);
            (add2(pos, DIRS[dir]), dir, false)
        }
        State::Flagged => {
            dir += 2;
            dir %= 4;
            map.entry(pos).and_modify(|b| *b = State::Clean);
            (add2(pos, DIRS[dir]), dir, false)
        }
    }
}

fn part2(input: &str) -> impl Debug {
    let mut map = input
        .lines()
        .enumerate()
        .flat_map(|(i, s)| {
            s.chars().enumerate().map(move |(j, c)| {
                (
                    (i as isize, j as isize),
                    if c == '#' {
                        State::Infected
                    } else {
                        State::Clean
                    },
                )
            })
        })
        .collect::<HashMap<_, _>>();

    let mut pos = (
        (input.lines().count() / 2) as isize,
        (input.lines().next().unwrap().chars().count() / 2) as isize,
    );
    let mut dir = 0;
    let mut out = 0;

    for _ in 0..10000000 {
        let res = burst2(pos, dir, &mut map);
        pos = res.0;
        dir = res.1;
        if res.2 {
            out += 1
        }
    }

    out
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/22")));
    println!("{:?}", part2(include_str!("../../input/22")));
}
