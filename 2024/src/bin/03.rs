use std::fmt::Debug;

use regex::Regex;

fn part1() -> impl Debug {
    let regex: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let input = include_str!("../../input/03");

    regex
        .captures_iter(input)
        .map(|c| {
            c.get(1).unwrap().as_str().parse::<i32>().unwrap()
                * c.get(2).unwrap().as_str().parse::<i32>().unwrap()
        })
        .sum::<i32>()
}

fn part2() -> impl Debug {
    let regex: Regex =
        Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let input = include_str!("../../input/03");

    let mut enabled = true;

    regex
        .captures_iter(input)
        .map(
            |c| match c.get(0).unwrap().as_str().chars().nth(2).unwrap() {
                '(' => {
                    enabled = true;
                    0
                }
                'n' => {
                    enabled = false;
                    0
                }
                'l' => {
                    if enabled {
                        c.get(1).unwrap().as_str().parse::<i32>().unwrap()
                            * c.get(2).unwrap().as_str().parse::<i32>().unwrap()
                    } else {
                        0
                    }
                }
                _ => panic!(),
            },
        )
        .sum::<i32>()
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}
