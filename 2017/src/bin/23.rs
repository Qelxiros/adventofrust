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
    let mut map = "abcdefgh"
        .chars()
        .map(|c| (c, 0))
        .collect::<HashMap<_, _>>();
    let mut pc = 0;

    let text = input.lines().collect_vec();

    let mut out = 0;

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
            "sub" => {
                *map.entry(chars[0]).or_insert(0) -= arg(args[1], &mut map)
            }
            "mul" => {
                out += 1;
                *map.entry(chars[0]).or_insert(0) *= arg(args[1], &mut map)
            }
            "jnz" => {
                if arg(args[0], &mut map) != 0 {
                    pc += arg(args[1], &mut map);
                    continue;
                }
            }
            _ => panic!(),
        }

        pc += 1;
    }

    out
}

fn part2(input: &str) -> impl Debug {
    let mut map = "abcdefgh"
        .chars()
        .map(|c| (c, 0))
        .collect::<HashMap<_, _>>();
    map.entry('a').and_modify(|v| *v = 1);
    let mut pc = 0;

    let text = input.lines().collect_vec();
    let increment =
        integers_unsigned::<usize>(input.lines().nth(30).unwrap())[0];

    while pc < text.len() as isize {
        if pc == 8 {
            break;
        }
        let s = text[pc as usize];
        let (opcode, args) = s.split_once(' ').unwrap();
        let args = args.split(' ').collect_vec();
        let chars =
            args.iter().map(|a| a.chars().next().unwrap()).collect_vec();
        match opcode {
            "set" => {
                *map.entry(chars[0]).or_insert(0) = arg(args[1], &mut map);
            }
            "sub" => {
                *map.entry(chars[0]).or_insert(0) -= arg(args[1], &mut map)
            }
            "mul" => {
                *map.entry(chars[0]).or_insert(0) *= arg(args[1], &mut map)
            }
            "jnz" => {
                if arg(args[0], &mut map) != 0 {
                    pc += arg(args[1], &mut map);
                    continue;
                }
            }
            _ => panic!(),
        }

        pc += 1;
    }

    (map[&'b']..=map[&'c'])
        .step_by(increment)
        .filter(|i| (2..*i).any(|x| i % x == 0))
        .count()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/23")));
    println!("{:?}", part2(include_str!("../../input/23")));
}
