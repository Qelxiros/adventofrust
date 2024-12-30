#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn arg(a: &str, map: &mut HashMap<char, isize>) -> isize {
    a.parse()
        .unwrap_or_else(|_| *map.entry(a.chars().next().unwrap()).or_insert(0))
}

fn part1(input: &str) -> impl Debug {
    let mut map = HashMap::new();
    let mut pc = 0;
    let mut sound = 0;

    let text = input.lines().collect_vec();

    while pc < text.len() as isize {
        let s = text[pc as usize];
        let (opcode, args) = s.split_once(' ').unwrap();
        let args = args.split(' ').collect_vec();
        let chars =
            args.iter().map(|a| a.chars().next().unwrap()).collect_vec();
        match opcode {
            "set" => {
                *map.entry(chars[0]).or_insert(0) = arg(args[1], &mut map);
            }
            "add" => {
                *map.entry(chars[0]).or_insert(0) += arg(args[1], &mut map)
            }
            "mul" => {
                *map.entry(chars[0]).or_insert(0) *= arg(args[1], &mut map)
            }
            "mod" => {
                *map.entry(chars[0]).or_insert(0) %= arg(args[1], &mut map)
            }
            "snd" => {
                sound = arg(args[0], &mut map);
            }
            "rcv" => {
                if arg(args[0], &mut map) != 0 {
                    return sound;
                }
            }
            "jgz" => {
                if arg(args[0], &mut map) > 0 {
                    pc += arg(args[1], &mut map);
                    continue;
                }
            }
            _ => panic!(),
        }

        pc += 1;
    }

    sound
}

fn part2(input: &str) -> impl Debug {
    let mut map = HashMap::new();
    let mut pc = 0;
    let mut v = VecDeque::new();

    let text = input.lines().take(21).collect_vec();

    while pc < text.len() as isize {
        let s = text[pc as usize];
        let (opcode, args) = s.split_once(' ').unwrap();
        let args = args.split(' ').collect_vec();
        let chars =
            args.iter().map(|a| a.chars().next().unwrap()).collect_vec();
        match opcode {
            "set" => {
                *map.entry(chars[0]).or_insert(0) = arg(args[1], &mut map);
            }
            "add" => {
                *map.entry(chars[0]).or_insert(0) += arg(args[1], &mut map)
            }
            "mul" => {
                *map.entry(chars[0]).or_insert(0) *= arg(args[1], &mut map)
            }
            "mod" => {
                *map.entry(chars[0]).or_insert(0) %= arg(args[1], &mut map)
            }
            "snd" => {
                v.push_back(arg(args[0], &mut map));
            }
            "jgz" => {
                if arg(args[0], &mut map) > 0 {
                    pc += arg(args[1], &mut map);
                    continue;
                }
            }
            _ => panic!(),
        }

        pc += 1;
    }

    let len = v.len();
    let mut out = len;
    let mut parity = 1;
    let mut idx = len - 1;

    while !v.iter().rev().is_sorted() {
        let i = v
            .iter()
            .take(idx + 1)
            .enumerate()
            .min_by_key(|(_, v)| **v)
            .unwrap()
            .0;
        let val = v.remove(i).unwrap();
        v.insert(idx, val);
        idx -= 1;
        if parity == 0 {
            out += len;
        }
        parity += 1;
        parity %= 2;
    }

    out
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/18")));
    println!("{:?}", part2(include_str!("../../input/18")));
}
