#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &str) -> impl Debug {
    let mut chunks = input.split("\n\n");
    let intro = chunks.next().unwrap();
    let states = chunks
        .map(|s| {
            let lines = s.lines().collect_vec();
            let state =
                lines[0].rsplit_once(' ').unwrap().1.chars().next().unwrap();
            let zero = (
                integers_unsigned::<usize>(lines[2])[0],
                lines[3].contains("right"),
                lines[4].rsplit_once(' ').unwrap().1.chars().next().unwrap(),
            );
            let one = (
                integers_unsigned::<usize>(lines[6])[0],
                lines[7].contains("right"),
                lines[8].rsplit_once(' ').unwrap().1.chars().next().unwrap(),
            );

            (state, (zero, one))
        })
        .collect::<HashMap<_, _>>();

    let steps = integers_unsigned::<usize>(intro)[0];
    let mut tape = HashMap::new();
    let mut state = intro
        .lines()
        .next()
        .unwrap()
        .rsplit_once(' ')
        .unwrap()
        .1
        .chars()
        .next()
        .unwrap();
    let mut pos = 0;

    for _ in 0..steps {
        let val = tape.entry(pos).or_insert(0);
        let s = if *val == 0 {
            states[&state].0
        } else {
            states[&state].1
        };
        *val = s.0;
        pos += if s.1 { 1 } else { -1 };
        state = s.2;
    }

    tape.into_iter().filter(|(k, v)| *v == 1).count()
}

fn part2(input: &str) -> impl Debug {}

fn main() {
    println!("{:?}", part1(include_str!("../../input/25")));
    println!("{:?}", part2(include_str!("../../input/25")));
}
