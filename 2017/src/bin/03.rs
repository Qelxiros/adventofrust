#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const DIRS8: [(isize, isize); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

// https://oeis.org/A214526
fn layer(num: usize) -> usize {
    ((num as f64).sqrt() * 0.5 - 0.5).ceil() as usize
}

fn dist(num: usize) -> usize {
    layer(num) + (num - 1).rem_euclid(2 * layer(num)).abs_diff(layer(num))
}

fn part1(input: &str) -> impl Debug {
    dist(input.trim().parse().unwrap())
}

struct Coords {
    x: isize,
    y: isize,
    dir: usize,
    cap: usize,
    used: usize,
    flag: bool,
}

impl Coords {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            dir: 0,
            cap: 1,
            used: 0,
            flag: false,
        }
    }
}

impl Iterator for Coords {
    type Item = (isize, isize);
    fn next(&mut self) -> Option<Self::Item> {
        let res = (self.x, self.y);

        if self.used == self.cap {
            self.dir = (self.dir + 1).rem_euclid(DIRS.len());
            self.used = 0;
            if self.flag {
                self.cap += 1;
                self.flag = false
            } else {
                self.flag = true;
            }
        }
        self.x += DIRS[self.dir].0;
        self.y += DIRS[self.dir].1;
        self.used += 1;

        Some(res)
    }
}

fn part2(input: &str) -> impl Debug {
    let mut map = HashMap::new();
    map.insert((0, 0), 1);
    let mut c = Coords::new().skip(1);
    let target = input.trim().parse().unwrap();

    loop {
        let pos = c.next().unwrap();
        let res = DIRS8
            .into_iter()
            .filter_map(|d| map.get(&add2(d, pos)))
            .sum::<usize>();
        map.insert(pos, res);

        if res > target {
            return res;
        }
    }
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/03")));
    println!("{:?}", part2(include_str!("../../input/03")));
}
