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

fn part1(input: &str) -> impl Debug {
    let mut map = HashMap::new();
    for c in "abcd".chars() {
        map.insert(c, 0);
    }

    let text = input.lines().collect_vec();
    let mut pc = 0;
    loop {
        if pc as usize >= text.len() {
            break;
        }
        let s = text[pc as usize];
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
            _ => panic!(),
        }
        pc += 1;
    }

    map[&'a']
}

fn part2(input: &str) -> impl Debug {
    let mut map = HashMap::new();
    for c in "abcd".chars() {
        map.insert(c, 0);
    }
    map.entry('c').and_modify(|a| *a = 1);

    let text = input.lines().collect_vec();
    let mut pc = 0;
    loop {
        if pc as usize >= text.len() {
            break;
        }
        let s = text[pc as usize];
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
            _ => panic!(),
        }
        pc += 1;
    }

    map[&'a']
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/12")));
    println!("{:?}", part2(include_str!("../../input/12")));
}
