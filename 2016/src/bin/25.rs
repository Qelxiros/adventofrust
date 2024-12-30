#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn arg(a: &str, registers: &HashMap<char, isize>) -> isize {
    a.parse()
        .unwrap_or_else(|_| registers[&a.chars().next().unwrap()])
}

fn process(input: &str, a: isize) -> bool {
    let mut map = HashMap::new();
    for c in "abcd".chars() {
        map.insert(c, 0);
    }
    map.insert('a', a);

    let text = input.lines().map(|s| s.to_owned()).collect_vec();
    let mut pc = 0;
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    seen.insert((
        0,
        map.iter()
            .sorted_by_key(|(a, _)| **a)
            .map(|(_, a)| a)
            .collect_vec(),
    ));
    loop {
        if pc as usize >= text.len() || out.len() == 10 {
            break;
        }
        let s = text[pc as usize].clone();
        let (inst, args) = s.split_once(' ').unwrap();
        match inst {
            "cpy" => {
                let args = args.split(' ').collect_vec();
                let val = arg(args[0], &map);
                map.insert(args[1].chars().next().unwrap(), val);
            }
            "inc" => {
                map.entry(args.chars().next().unwrap())
                    .and_modify(|a| *a += 1);
            }
            "dec" => {
                map.entry(args.chars().next().unwrap())
                    .and_modify(|a| *a -= 1);
            }
            "jnz" => {
                let args = args.split(' ').collect_vec();
                if arg(args[0], &map) != 0 {
                    pc += arg(args[1], &map);
                    continue;
                }
            }
            "out" => {
                let x = arg(args, &map);
                if vec![out.last().unwrap_or(&1), &x]
                    .into_iter()
                    .cloned()
                    .sorted()
                    .collect_vec()
                    != vec![0, 1]
                {
                    return false;
                }
                out.push(x);
            }
            _ => panic!(),
        }
        pc += 1;
    }

    (0..=1).cycle().take(out.len()).collect_vec() == out
}

fn part1(input: &str) -> impl Debug {
    (0..isize::MAX).find(|a| process(input, *a)).unwrap()
}

fn part2(input: &str) -> impl Debug {}

fn main() {
    println!("{:?}", part1(include_str!("../../input/25")));
    println!("{:?}", part2(include_str!("../../input/25")));
}
