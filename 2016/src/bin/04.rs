#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn part1(input: &str) -> impl Debug {
    input
        .lines()
        .map(|s| s.split_once('[').unwrap())
        .map(|(room, ck)| {
            let (room, id) = room.rsplit_once('-').unwrap();
            if ck.starts_with(
                &room
                    .chars()
                    .filter(|c| *c != '-')
                    .sorted()
                    .dedup_with_count()
                    .sorted_by(|(cnt1, ch1), (cnt2, ch2)| {
                        cnt1.cmp(cnt2).then_with(|| ch2.cmp(ch1))
                    })
                    .rev()
                    .take(5)
                    .map(|(_, c)| c)
                    .collect::<String>(),
            ) {
                id.parse::<usize>().unwrap()
            } else {
                0
            }
        })
        .sum::<usize>()
}

fn part2(input: &str) -> impl Debug {
    input
        .lines()
        .map(|s| s.split_once('[').unwrap())
        .filter_map(|(room, ck)| {
            let (room, id) = room.rsplit_once('-').unwrap();
            if ck.starts_with(
                &room
                    .chars()
                    .filter(|c| *c != '-')
                    .sorted()
                    .dedup_with_count()
                    .sorted_by(|(cnt1, ch1), (cnt2, ch2)| {
                        cnt1.cmp(cnt2).then_with(|| ch2.cmp(ch1))
                    })
                    .rev()
                    .take(5)
                    .map(|(_, c)| c)
                    .collect::<String>(),
            ) {
                Some((room, id.parse::<u32>().unwrap()))
            } else {
                None
            }
        })
        .map(|(room, id)| {
            (
                id,
                room.chars()
                    .map(|c| ((c as u32 - 97u32 + id) % 26 + 97) as u8 as char)
                    .collect::<String>(),
            )
        })
        .collect_vec()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/04")));
    println!("{:?}", part2(include_str!("../../input/04")));
}
