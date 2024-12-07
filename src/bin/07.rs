use std::{fs, ops::BitAnd};

use anyhow::Result;
use itertools::{Itertools, repeat_n};
use regex::Regex;

fn check_eq(eq: (i64, Vec<i64>)) -> bool {
    for bits in 0..2isize.pow(eq.1.len() as u32 - 1) {
        let mut total = eq.1[0];
        for (idx, num) in eq.1[1..].iter().enumerate() {
            if bits.bitand(2isize.pow(idx as u32)) > 0 {
                total += num;
            } else {
                total *= num;
            }
        }
        if total == eq.0 {
            return true;
        }
    }
    false
}

fn part1() -> Result<()> {
    let input = include_str!("/home/jeremy/github/adventofrust/input/07");

    let eqs = input
        .lines()
        .map(|s| s.split(": ").collect_vec())
        .map(|v| {
            (
                v[0].parse::<i64>().unwrap(),
                v[1].split(' ')
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect_vec(),
            )
        })
        .filter(|a| check_eq(a.clone()))
        .map(|a| a.0)
        .sum::<i64>();

    println!("{eqs}");

    Ok(())
}

fn check_eq_sane(eq: (i64, Vec<i64>)) -> bool {
    repeat_n(0..3, eq.1.len() - 1)
        .multi_cartesian_product()
        .any(|c| {
            let mut total = eq.1[0];
            for (idx, num) in eq.1[1..].iter().enumerate() {
                match c[idx] {
                    0 => total += num,
                    1 => total *= num,
                    2 => {
                        let mut s = total.to_string();
                        s.push_str(&num.to_string());
                        total = s.parse().unwrap();
                    }
                    _ => panic!(),
                }
            }
            total == eq.0
        })
}

fn part2() -> Result<()> {
    let input = include_str!("/home/jeremy/github/adventofrust/input/07");

    let eqs = input
        .lines()
        .map(|s| s.split(": ").collect_vec())
        .map(|v| {
            (
                v[0].parse::<i64>().unwrap(),
                v[1].split(' ')
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect_vec(),
            )
        })
        .filter(|a| check_eq_sane(a.clone()))
        .map(|a| a.0)
        .sum::<i64>();

    println!("{eqs}");

    Ok(())
}

fn main() {
    part1().unwrap();
    part2().unwrap();
}
