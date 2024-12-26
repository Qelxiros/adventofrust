use std::fmt::Debug;

use itertools::{Itertools, repeat_n};

fn check_eq_sane(eq: (i64, Vec<i64>), ops: usize) -> bool {
    repeat_n(0..ops, eq.1.len() - 1)
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

fn part1() -> impl Debug {
    let input = include_str!("../../input/07");

    input
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
        .filter(|a| check_eq_sane(a.clone(), 2))
        .map(|a| a.0)
        .sum::<i64>()
}

fn part2() -> impl Debug {
    let input = include_str!("../../input/07");

    input
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
        .filter(|a| check_eq_sane(a.clone(), 3))
        .map(|a| a.0)
        .sum::<i64>()
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}
