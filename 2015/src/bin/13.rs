#![feature(let_chains)]

use std::{cmp::Reverse, collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn part1(input: &str) -> impl Debug {
    let mut people = HashSet::new();
    let map = input
        .lines()
        .map(|s| s.split_whitespace().collect_vec())
        .map(|v| {
            let a = v[0].chars().next().unwrap();
            let b = v[10].chars().next().unwrap();
            people.insert(a);
            people.insert(b);
            (
                (a, b),
                v[3].parse::<isize>().unwrap()
                    * if v[2] == "lose" { -1 } else { 1 },
            )
        })
        .collect::<HashMap<_, _>>();

    let start = (0, vec!['A']);
    let mut q = BinaryHeap::new();
    q.push(start);
    let mut out = 0;
    while let Some((c, v)) = q.pop() {
        for p in &people {
            if v.contains(p) {
                continue;
            }
            let mut cost = c
                + map.get(&(*v.last().unwrap(), *p)).unwrap()
                + map.get(&(*p, *v.last().unwrap())).unwrap();
            let mut new = v.clone();
            new.push(*p);
            if new.len() == people.len() {
                cost += map
                    .get(&(*new.last().unwrap(), *new.first().unwrap()))
                    .unwrap()
                    + map
                        .get(&(*new.first().unwrap(), *new.last().unwrap()))
                        .unwrap();
                out = out.max(cost);
            }
            q.push((cost, new));
        }
    }

    out
}

fn part2(input: &str) -> impl Debug {
    let mut people = HashSet::new();
    let mut map = input
        .lines()
        .map(|s| s.split_whitespace().collect_vec())
        .map(|v| {
            let a = v[0].chars().next().unwrap();
            let b = v[10].chars().next().unwrap();
            people.insert(a);
            people.insert(b);
            (
                (a, b),
                v[3].parse::<isize>().unwrap()
                    * if v[2] == "lose" { -1 } else { 1 },
            )
        })
        .collect::<HashMap<_, _>>();
    people.iter().for_each(|p| {
        map.insert(('Y', *p), 0);
        map.insert((*p, 'Y'), 0);
    });
    people.insert('Y');

    let start = (0, vec!['A']);
    let mut q = BinaryHeap::new();
    q.push(start);
    let mut out = 0;
    while let Some((c, v)) = q.pop() {
        for p in &people {
            if v.contains(p) {
                continue;
            }
            let mut cost = c
                + map.get(&(*v.last().unwrap(), *p)).unwrap()
                + map.get(&(*p, *v.last().unwrap())).unwrap();
            let mut new = v.clone();
            new.push(*p);
            if new.len() == people.len() {
                cost += map
                    .get(&(*new.last().unwrap(), *new.first().unwrap()))
                    .unwrap()
                    + map
                        .get(&(*new.first().unwrap(), *new.last().unwrap()))
                        .unwrap();
                out = out.max(cost);
            }
            q.push((cost, new));
        }
    }

    out
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/13")));
    println!("{:?}", part2(include_str!("../../input/13")));
}
