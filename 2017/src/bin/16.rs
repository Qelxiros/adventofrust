#![feature(let_chains)]
#![feature(slice_swap_unchecked)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &str) -> impl Debug {
    let mut programs = "abcdefghijklmnop".chars().collect_vec();

    for line in input.trim().split(',') {
        match line.chars().next().unwrap() {
            's' => programs.rotate_right(integers_unsigned(line)[0]),
            'x' => {
                let v = integers_unsigned(line);
                unsafe { programs.swap_unchecked(v[0], v[1]) }
            }
            'p' => {
                let v = line.chars().collect_vec();
                let a = programs.iter().position(|c| *c == v[1]).unwrap();
                let b = programs.iter().position(|c| *c == v[3]).unwrap();
                programs.swap(a, b)
            }
            _ => panic!(),
        }
    }

    programs.into_iter().collect::<String>()
}

fn part2(input: &str) -> impl Debug {
    let original = "abcdefghijklmnop".chars().collect_vec();
    let mut programs = original.clone();

    let mut i = 0;
    while i < 1_000_000_000 {
        for line in input.trim().split(',') {
            match line.chars().next().unwrap() {
                's' => programs.rotate_right(integers_unsigned(line)[0]),
                'x' => {
                    let v = integers_unsigned(line);
                    unsafe { programs.swap_unchecked(v[0], v[1]) }
                }
                'p' => {
                    let v = line.chars().collect_vec();
                    let a = programs.iter().position(|c| *c == v[1]).unwrap();
                    let b = programs.iter().position(|c| *c == v[3]).unwrap();
                    programs.swap(a, b)
                }
                _ => panic!(),
            }
        }
        i += 1;
        if programs == original {
            i += (1_000_000_000 - i) / i * i;
        }
    }

    programs.into_iter().collect::<String>()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/16")));
    println!("{:?}", part2(include_str!("../../input/16")));
}
