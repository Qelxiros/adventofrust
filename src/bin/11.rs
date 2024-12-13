use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

fn part1(len: usize, v: Option<Vec<isize>>) -> usize {
    let input = include_str!("/home/jeremy/github/adventofrust/input/11");

    let mut stones = v.unwrap_or_else(|| {
        input
            .split(' ')
            .filter_map(|s| s.trim().parse::<isize>().ok())
            .collect_vec()
    });

    for i in 0..len {
        stones = stones
            .into_iter()
            .flat_map(|r| match r {
                0 => vec![1],
                r => {
                    let len = (r as f64).log10().floor() as u32 + 1;
                    if len % 2 == 0 {
                        vec![r / 10isize.pow(len / 2), r % 10isize.pow(len / 2)]
                    } else {
                        vec![r * 2024]
                    }
                }
            })
            .collect_vec();
    }

    // println!("{}", stones.len());

    stones.len()
}

fn blink(num: usize, blinks: usize, map: &mut HashMap<(usize, usize), usize>) -> usize {
    if blinks == 0 {
        return 1;
    }
    if let Some(v) = map.get(&(num, blinks)) {
        return *v;
    }
    let vec = match num {
        0 => vec![1],
        r => {
            let len = (r as f64).log10().floor() as u32 + 1;
            if len % 2 == 0 {
                vec![r / 10usize.pow(len / 2), r % 10usize.pow(len / 2)]
            } else {
                vec![r * 2024]
            }
        }
    };

    let out = vec.into_iter().map(|n| blink(n, blinks - 1, map)).sum();

    map.insert((num, blinks), out);

    out
}

fn part2() -> Result<()> {
    let input = include_str!("/home/jeremy/github/adventofrust/input/11");
    let len = 25;

    let mut stones = input
        .split(' ')
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect_vec();
    let mut total = 0;

    let mut map = HashMap::new();

    println!(
        "{}",
        stones
            .into_iter()
            .map(|s| blink(s, 75, &mut map))
            .sum::<usize>()
    );

    Ok(())
}

fn main() {
    part1(25, None);
    part2().unwrap();
}
