#![feature(let_chains)]

use std::collections::*;

use itertools::Itertools;
use regex::Regex;

use adventofrust::utils::*;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn part1() {
    let input = include_str!("/home/jeremy/github/adventofrust/input/18");

    let bytes = input
        .lines()
        .take(1024)
        .map(|s| {
            s.split(',')
                .take(2)
                .map(str::parse::<isize>)
                .map(Result::unwrap)
                .collect_vec()
        })
        .map(|v| (v[0], v[1]))
        .collect::<HashSet<_>>();

    let mut q = VecDeque::new();
    q.push_back(((0isize, 0isize), 0));

    let mut seen = HashSet::new();

    while let Some((e, steps)) = q.pop_front() {
        if e == (70, 70) {
            println!("{steps}");
            return;
        }
        if seen.contains(&e) {
            continue;
        }
        seen.insert(e);
        for d in DIRS {
            let new = (e.0 + d.0, e.1 + d.1);
            if new.0 >= 0 && new.0 <= 70 && new.1 >= 0 && new.1 <= 70 && !bytes.contains(&new) {
                q.push_back((new, steps + 1));
            }
        }
    }
}

fn part2() {
    let input = include_str!("/home/jeremy/github/adventofrust/input/18");

    let bytes = input
        .lines()
        .map(|s| {
            s.split(',')
                .take(2)
                .map(str::parse::<isize>)
                .map(Result::unwrap)
                .collect_vec()
        })
        .map(|v| (v[0], v[1]))
        .collect_vec();

    let mut idx = 0;

    'outer: loop {
        let b = bytes.iter().take(idx).collect::<HashSet<_>>();
        let mut q = VecDeque::new();
        q.push_back(((0isize, 0isize), 0));

        let mut seen = HashSet::new();
        let mut pred = HashMap::new();

        while let Some((mut e, steps)) = q.pop_front() {
            if e == (70, 70) {
                let mut path = HashSet::new();
                while let Some(&p) = pred.get(&e) {
                    path.insert(p);
                    e = p;
                }
                idx = bytes.iter().position(|a| path.contains(a)).unwrap() + 1;
                continue 'outer;
            }
            if seen.contains(&e) {
                continue;
            }
            seen.insert(e);
            for d in DIRS {
                let new = (e.0 + d.0, e.1 + d.1);
                if new.0 >= 0
                    && new.0 <= 70
                    && new.1 >= 0
                    && new.1 <= 70
                    && !b.contains(&new)
                    && !seen.contains(&new)
                {
                    q.push_back((new, steps + 1));
                    pred.insert(new, e);
                }
            }
        }
        println!("{:?}", bytes[idx - 1]);
        break;
    }
}

fn main() {
    part1();
    part2();
}
