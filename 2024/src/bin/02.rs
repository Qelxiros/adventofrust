use std::fmt::Debug;

fn part1() -> impl Debug {
    let input = include_str!("../../input/02");

    input
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
        .count()
}

fn part2() -> impl Debug {
    let input = include_str!("../../input/02");

    input
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
        .count()
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}
