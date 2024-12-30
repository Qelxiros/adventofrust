#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn do_inst(inst: &str, pass: &mut [char]) {
    match inst {
        i if i.starts_with("swap p") => {
            let v = integers_unsigned::<usize>(i);
            pass.swap(v[0], v[1]);
        }
        i if i.starts_with("swap l") => {
            let letters = i
                .split_whitespace()
                .map(|s| s.chars().next().unwrap())
                .collect_vec();
            let p = (
                pass.iter().position(|&c| c == letters[2]).unwrap(),
                pass.iter().position(|&c| c == letters[5]).unwrap(),
            );
            pass.swap(p.0, p.1);
        }
        i if i.starts_with("rotate l") => {
            let v = integers_unsigned::<usize>(i);
            pass.rotate_left(v[0]);
        }
        i if i.starts_with("rotate r") => {
            let v = integers_unsigned::<usize>(i);
            pass.rotate_right(v[0]);
        }
        i if i.starts_with("rotate b") => {
            let letter = i.split_whitespace().last().unwrap();
            let pos = pass.iter().position(|c| letter.starts_with(*c)).unwrap();
            let idx =
                (pos + 1 + if pos >= 4 { 1 } else { 0 }).rem_euclid(pass.len());
            pass.rotate_right(idx);
        }
        i if i.starts_with("reverse") => {
            let v = integers_unsigned::<usize>(i);
            pass[v[0]..v[1] + 1].reverse();
        }
        i if i.starts_with("move") => {
            let v = integers_unsigned::<usize>(i);
            if v[0] < v[1] {
                pass[v[0]..v[1] + 1].rotate_left(1);
            } else {
                pass[v[1]..v[0] + 1].rotate_right(1);
            }
        }
        _ => panic!(),
    }
}

fn part1(input: &str) -> impl Debug {
    let mut pass = "abcdefgh".chars().collect_vec();

    for inst in input.lines() {
        do_inst(inst, &mut pass);
        println!("{}", pass.iter().join(""))
    }

    pass.into_iter().join("")
}

fn rev_inst(inst: &str, pass: &mut [char]) {
    match inst {
        i if i.starts_with("swap p") => {
            let v = integers_unsigned::<usize>(i);
            pass.swap(v[0], v[1]);
        }
        i if i.starts_with("swap l") => {
            let letters = i
                .split_whitespace()
                .map(|s| s.chars().next().unwrap())
                .collect_vec();
            let p = (
                pass.iter().position(|&c| c == letters[2]).unwrap(),
                pass.iter().position(|&c| c == letters[5]).unwrap(),
            );
            pass.swap(p.0, p.1);
        }
        i if i.starts_with("rotate l") => {
            let v = integers_unsigned::<usize>(i);
            pass.rotate_right(v[0]);
        }
        i if i.starts_with("rotate r") => {
            let v = integers_unsigned::<usize>(i);
            pass.rotate_left(v[0]);
        }
        i if i.starts_with("rotate b") => {
            let map = [1, 1, 6, 2, 7, 3, 0, 4];
            let letter = i.split_whitespace().last().unwrap();
            let pos = pass.iter().position(|c| letter.starts_with(*c)).unwrap();
            let idx = map[pos];
            pass.rotate_left(idx);
        }
        i if i.starts_with("reverse") => {
            let v = integers_unsigned::<usize>(i);
            pass[v[0]..=v[1]].reverse();
        }
        i if i.starts_with("move") => {
            let v = integers_unsigned::<usize>(i);
            if v[0] < v[1] {
                pass[v[0]..=v[1]].rotate_right(1);
            } else {
                pass[v[1]..=v[0]].rotate_left(1);
            }
        }
        _ => panic!(),
    }
}

fn part2(input: &str) -> impl Debug {
    let mut pass = "fbgdceah".chars().collect_vec();

    for inst in input.lines().rev() {
        rev_inst(inst, &mut pass);
    }

    pass.into_iter().join("")
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/21")));
    println!("{:?}", part2(include_str!("../../input/21")));
}
