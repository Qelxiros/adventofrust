use std::{cmp::Ordering, fmt::Debug};

fn part1() -> impl Debug {
    let input = include_str!("../../input/05");

    let rules = input
        .lines()
        .take_while(|s| !s.is_empty())
        .map(|s| s.split('|').collect::<Vec<_>>())
        .map(|v| (v[0], v[1]))
        .collect::<Vec<_>>();
    input
        .lines()
        .skip_while(|s| !s.is_empty())
        .skip(1)
        .map(|s| s.split(',').collect::<Vec<_>>())
        .filter(|pages| {
            for rule in &rules {
                if let Some(x) = pages.iter().position(|&s| s == rule.0) {
                    if let Some(y) = pages.iter().position(|&s| s == rule.1) {
                        if x > y {
                            return false;
                        }
                    }
                }
            }
            true
        })
        .map(|pages| pages[(pages.len() - 1) / 2].parse::<i32>().unwrap())
        .sum::<i32>()
}

fn part2() -> impl Debug {
    let input = include_str!("../../input/05");

    let rules = input
        .lines()
        .take_while(|s| !s.is_empty())
        .map(|s| s.split('|').collect::<Vec<_>>())
        .map(|v| (v[0].parse::<i32>().unwrap(), v[1].parse::<i32>().unwrap()))
        .collect::<Vec<_>>();
    input
        .lines()
        .skip_while(|s| !s.is_empty())
        .skip(1)
        .map(|s| {
            s.split(',')
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|pages| {
            for rule in &rules {
                if let Some(x) = pages.iter().position(|&s| s == rule.0) {
                    if let Some(y) = pages.iter().position(|&s| s == rule.1) {
                        if x > y {
                            return true;
                        }
                    }
                }
            }
            false
        })
        .map(|mut pages| {
            pages.sort_unstable_by(|&a, &b| {
                for rule in &rules {
                    if rule.0 == a && rule.1 == b {
                        return Ordering::Less;
                    } else if rule.0 == b && rule.1 == a {
                        return Ordering::Greater;
                    }
                }
                Ordering::Equal
            });
            pages
        })
        .map(|pages| pages[(pages.len() - 1) / 2])
        .sum::<i32>()
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}
