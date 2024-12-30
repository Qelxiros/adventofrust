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
    map.insert('a', 7);

    let mut text = input.lines().map(|s| s.to_owned()).collect_vec();
    let mut pc = 0;
    loop {
        if pc as usize >= text.len() {
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
            "tgl" => {
                if let Some(i) =
                    text.get_mut(pc as usize + arg(args, &map) as usize)
                {
                    let (inst_inner, a) = i.split_once(' ').unwrap();
                    let inst_inner = match inst_inner {
                        "cpy" => "jnz",
                        "jnz" => "cpy",
                        "inc" => "dec",
                        "dec" | "tgl" => "inc",
                        _ => panic!(),
                    };
                    *i = format!("{inst_inner} {a}");
                }
            }
            _ => panic!(),
        }
        pc += 1;
    }

    map[&'a']
}

fn factorial(i: usize) -> usize {
    (1..=i).product()
}

fn part2(input: &str) -> impl Debug {
    input
        .lines()
        .skip(19)
        .take(2)
        .map(|s| integers_unsigned::<usize>(s)[0])
        .product::<usize>()
        + factorial(12)
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/23")));
    println!("{:?}", part2(include_str!("../../input/23")));
}
