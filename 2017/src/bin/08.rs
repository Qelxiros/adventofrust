#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn get_val<'a>(reg: &'a str, map: &mut HashMap<&'a str, isize>) -> isize {
    *map.entry(reg).or_insert(0)
}

fn part1(input: &str) -> impl Debug {
    let mut map = HashMap::new();

    input.lines().for_each(|s| {
        let v = s.split_whitespace().collect_vec();
        let a = get_val(v[4], &mut map);
        let b = v[6].parse::<isize>().unwrap();
        let cond = match v[5] {
            ">" => a > b,
            "<" => a < b,
            ">=" => a >= b,
            "<=" => a <= b,
            "==" => a == b,
            "!=" => a != b,
            _ => panic!(),
        };
        if cond {
            let c = v[2].parse::<isize>().unwrap();
            *map.entry(v[0]).or_insert(0) +=
                if v[1].starts_with('i') { c } else { -c };
        }
    });

    *map.values().max().unwrap()
}

fn part2(input: &str) -> impl Debug {
    let mut map = HashMap::new();
    let mut max = 0;

    input.lines().for_each(|s| {
        let v = s.split_whitespace().collect_vec();
        let a = get_val(v[4], &mut map);
        let b = v[6].parse::<isize>().unwrap();
        let cond = match v[5] {
            ">" => a > b,
            "<" => a < b,
            ">=" => a >= b,
            "<=" => a <= b,
            "==" => a == b,
            "!=" => a != b,
            _ => panic!(),
        };
        if cond {
            let c = v[2].parse::<isize>().unwrap();
            *map.entry(v[0]).or_insert(0) +=
                if v[1].starts_with('i') { c } else { -c };
            max = max.max(map[&v[0]]);
        }
    });

    max
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/08")));
    println!("{:?}", part2(include_str!("../../input/08")));
}
