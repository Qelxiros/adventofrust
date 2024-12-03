use std::fs;

use anyhow::Result;
use regex::Regex;

fn part1() -> Result<()> {
    let regex: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let input = fs::read_to_string("/home/jeremy/github/adventofrust/input/03")?;

    let out = regex
        .captures_iter(&input)
        .map(|c| {
            c.get(1).unwrap().as_str().parse::<i32>().unwrap()
                * c.get(2).unwrap().as_str().parse::<i32>().unwrap()
        })
        .sum::<i32>();

    println!("{out}");

    Ok(())
}

fn part2() -> Result<()> {
    let regex: Regex = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let input = fs::read_to_string("/home/jeremy/github/adventofrust/input/03")?;

    let mut enabled = true;

    let out = regex
        .captures_iter(&input)
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
        .sum::<i32>();

    println!("{out}");

    Ok(())
}

fn main() {
    part1().unwrap();
    part2().unwrap();
}
