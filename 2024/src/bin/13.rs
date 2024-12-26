use std::fmt::Debug;

use regex::Regex;

fn part1() -> impl Debug {
    let input = include_str!("../../input/13");

    let re = Regex::new(r"\d+").unwrap();

    input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .filter_map(|s| {
            let v = re
                .find_iter(s)
                .map(|m| m.as_str().parse::<usize>().unwrap())
                .take(6)
                .collect::<Vec<_>>();
            find((v[4], v[5]), (v[0], v[1]), (v[2], v[3]))
        })
        .sum::<usize>()
}

fn find(
    c: (usize, usize),
    a: (usize, usize),
    b: (usize, usize),
) -> Option<usize> {
    let d = (a.0 * b.1) as isize - (a.1 * b.0) as isize;
    if d == 0 {
        return None;
    }

    let x = (b.1 * c.0) as isize - (b.0 * c.1) as isize;
    let y = (a.0 * c.1) as isize - (a.1 * c.0) as isize;

    if x % d != 0 || y % d != 0 {
        None
    } else {
        let r = 3 * x / d + y / d;
        if r < 0 { None } else { Some(r as usize) }
    }
}

fn part2() -> impl Debug {
    let input = include_str!("../../input/13");

    let re = Regex::new(r"\d+").unwrap();

    input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .filter_map(|s| {
            let v = re
                .find_iter(s)
                .map(|m| m.as_str().parse::<usize>().unwrap())
                .take(6)
                .collect::<Vec<_>>();
            find(
                (v[4] + 10000000000000, v[5] + 10000000000000),
                (v[0], v[1]),
                (v[2], v[3]),
            )
        })
        .sum::<usize>()
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}
