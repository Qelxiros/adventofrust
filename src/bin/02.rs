use std::fs;

use anyhow::Result;

fn part1() -> Result<()> {
    let input = fs::read_to_string("/home/jeremy/github/adventofrust/input/02")?;

    let out = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|v| {
            let inc = v[0] < v[1];
            v.iter().enumerate().skip(1).all(|(idx, val)| {
                1 <= v[idx - 1].abs_diff(*val)
                    && v[idx - 1].abs_diff(*val) <= 3
                    && if inc {
                        *val > v[idx - 1]
                    } else {
                        *val < v[idx - 1]
                    }
            })
        })
        .count();

    println!("{out}");

    Ok(())
}

fn part2() -> Result<()> {
    let input = fs::read_to_string("/home/jeremy/github/adventofrust/input/02")?;

    let out = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|v| {
            for r in 0..v.len() {
                let v = v
                    .iter()
                    .enumerate()
                    .filter(|(idx, _)| *idx != r)
                    .map(|(_, v)| v)
                    .collect::<Vec<_>>();
                let inc = v[0] < v[1];
                if v.iter().enumerate().skip(1).all(|(idx, val)| {
                    1 <= v[idx - 1].abs_diff(**val)
                        && v[idx - 1].abs_diff(**val) <= 3
                        && if inc {
                            *val > v[idx - 1]
                        } else {
                            *val < v[idx - 1]
                        }
                }) {
                    return true;
                }
            }
            false
        })
        .count();

    println!("{out}");

    Ok(())
}

fn main() {
    // part1().unwrap();
    part2().unwrap();
}
