#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn part1(input: &str) -> impl Debug {
    let mut a = 0;
    let mut b = 0;
    let text = input
        .lines()
        .map(|s| s.split_once(" ").unwrap())
        .collect_vec();
    let mut pc = 0;
    loop {
        if pc >= text.len() {
            break;
        }
        let (inst, args) = text[pc];
        match inst {
            "hlf" => {
                if args == "a" {
                    a /= 2;
                } else {
                    b /= 2;
                }
            }
            "tpl" => {
                if args == "a" {
                    a *= 3;
                } else {
                    b *= 3;
                }
            }
            "inc" => {
                if args == "a" {
                    a += 1;
                } else {
                    b += 1;
                }
            }
            "jmp" => {
                pc = (pc as isize + args.trim().parse::<isize>().unwrap())
                    as usize;
                continue;
            }
            "jie" => {
                let args = args.split(", ").map(str::trim).collect_vec();
                if args[0] == "a" && a % 2 == 0 {
                    pc = (pc as isize + args[1].parse::<isize>().unwrap())
                        as usize;
                    continue;
                }
            }
            "jio" => {
                let args = args.split(", ").map(str::trim).collect_vec();
                if args[0] == "a" && a == 1 {
                    pc = (pc as isize + args[1].parse::<isize>().unwrap())
                        as usize;
                    continue;
                }
            }
            _ => panic!(),
        }
        pc += 1;
    }

    b
}

fn part2(input: &str) -> impl Debug {
    let mut a = 1;
    let mut b = 0;
    let text = input
        .lines()
        .map(|s| s.split_once(" ").unwrap())
        .collect_vec();
    let mut pc = 0;
    loop {
        if pc >= text.len() {
            break;
        }
        let (inst, args) = text[pc];
        match inst {
            "hlf" => {
                if args == "a" {
                    a /= 2;
                } else {
                    b /= 2;
                }
            }
            "tpl" => {
                if args == "a" {
                    a *= 3;
                } else {
                    b *= 3;
                }
            }
            "inc" => {
                if args == "a" {
                    a += 1;
                } else {
                    b += 1;
                }
            }
            "jmp" => {
                pc = (pc as isize + args.trim().parse::<isize>().unwrap())
                    as usize;
                continue;
            }
            "jie" => {
                let args = args.split(", ").map(str::trim).collect_vec();
                if args[0] == "a" && a % 2 == 0 {
                    pc = (pc as isize + args[1].parse::<isize>().unwrap())
                        as usize;
                    continue;
                }
            }
            "jio" => {
                let args = args.split(", ").map(str::trim).collect_vec();
                if args[0] == "a" && a == 1 {
                    pc = (pc as isize + args[1].parse::<isize>().unwrap())
                        as usize;
                    continue;
                }
            }
            _ => panic!(),
        }
        pc += 1;
    }

    b
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/23")));
    println!("{:?}", part2(include_str!("../../input/23")));
}
