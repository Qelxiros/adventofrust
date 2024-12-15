use std::collections::HashSet;

use adventofrust::utils::*;
use itertools::Itertools;
use regex::Regex;

fn part1() {
    let input = include_str!("/home/jeremy/github/adventofrust/input/14");

    let (mut a, mut b, mut c, mut d) = (0, 0, 0, 0);

    let re = Regex::new(r"p=(.*),(.*) v=(.*),(.*)").unwrap();

    input
        .lines()
        .map(|s| {
            re.captures(s)
                .map(|c| {
                    (
                        c[1].parse::<isize>().unwrap(),
                        c[2].parse::<isize>().unwrap(),
                        c[3].parse::<isize>().unwrap(),
                        c[4].parse::<isize>().unwrap(),
                    )
                })
                .unwrap()
        })
        .map(|(x0, y0, vx, vy)| {
            (
                (x0 + 100 * vx).rem_euclid(101),
                (y0 + 100 * vy).rem_euclid(103),
            )
        })
        .for_each(|(x, y)| match (x, y) {
            (0..=49, 0..=50) => a += 1,
            (51..=100, 0..=50) => b += 1,
            (0..=49, 52..=102) => c += 1,
            (51..=100, 52..=102) => d += 1,
            _ => (),
        });

    println!("{a},{b},{c},{d}");
    println!("{}", a * b * c * d);
}

fn print(set: &HashSet<(isize, isize)>) {
    for i in 0..101 {
        let mut s = String::new();
        for j in 0..103 {
            if set.contains(&(i, j)) {
                s.push('X');
            } else {
                s.push(' ');
            }
        }
        println!("{s}");
    }
}

fn part2() {
    let input = include_str!("/home/jeremy/github/adventofrust/input/14");

    let re = Regex::new(r"p=(.*),(.*) v=(.*),(.*)").unwrap();

    let mut robots = input
        .lines()
        .map(|s| {
            re.captures(s)
                .map(|c| {
                    (
                        c[1].parse::<isize>().unwrap(),
                        c[2].parse::<isize>().unwrap(),
                        c[3].parse::<isize>().unwrap(),
                        c[4].parse::<isize>().unwrap(),
                    )
                })
                .unwrap()
        })
        .collect_vec();

    let mut cnt = 0;
    for i in 0..101 {
        for j in 0..103 {
            robots.iter_mut().for_each(|(x, y, vx, vy)| {
                *x += *vx;
                *x = x.rem_euclid(101);
                *y += *vy;
                *y = y.rem_euclid(103);
            });
            cnt += 1;
            let set = robots
                .iter()
                .map(|(a, b, _, _)| (*a, *b))
                .collect::<HashSet<_>>();

            println!("{cnt}");
            print(&set);
        }
    }
}

fn main() {
    part1();
    // part2();
}
