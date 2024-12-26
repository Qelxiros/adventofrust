use std::{collections::HashMap, fmt::Debug};

use itertools::Itertools;

fn blink(
    num: usize,
    blinks: usize,
    map: &mut HashMap<(usize, usize), usize>,
) -> usize {
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

fn part1() -> impl Debug {
    let input = include_str!("../../input/11");

    let stones = input
        .split(' ')
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect_vec();

    let mut map = HashMap::new();

    stones
        .into_iter()
        .map(|s| blink(s, 25, &mut map))
        .sum::<usize>()
}

fn part2() -> impl Debug {
    let input = include_str!("../../input/11");

    let stones = input
        .split(' ')
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect_vec();

    let mut map = HashMap::new();

    stones
        .into_iter()
        .map(|s| blink(s, 75, &mut map))
        .sum::<usize>()
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}
