#![feature(let_chains)]

use std::{collections::*, fmt::Debug, mem::swap};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &str) -> impl Debug {
    let mut screen = vec![vec![false; 50]; 6];

    input.lines().for_each(|s| match s.chars().nth(1).unwrap() {
        'e' => {
            let v = integers_unsigned::<usize>(s);
            screen.iter_mut().take(v[1]).for_each(|row| {
                row.iter_mut().take(v[0]).for_each(|p| *p = true)
            });
        }
        'o' => {
            let v = integers_unsigned::<usize>(s);
            if s.chars().nth(7).unwrap() == 'r' {
                let row = screen[v[0]].clone();
                screen[v[0]] = row[row.len() - v[1]..].to_vec();
                screen[v[0]].extend(&row[..row.len() - v[1]]);
            } else {
                for _ in 0..v[1] {
                    let mut temp = false;
                    for i in 0..screen.len() {
                        swap(&mut screen[i][v[0]], &mut temp);
                    }
                    swap(&mut screen[0][v[0]], &mut temp);
                }
            }
        }
        _ => panic!(),
    });

    screen.into_iter().flatten().filter(|a| *a).count()
}

fn part2(input: &str) -> impl Debug {
    let mut screen = vec![vec![false; 50]; 6];

    input.lines().for_each(|s| match s.chars().nth(1).unwrap() {
        'e' => {
            let v = integers_unsigned::<usize>(s);
            screen.iter_mut().take(v[1]).for_each(|row| {
                row.iter_mut().take(v[0]).for_each(|p| *p = true)
            });
        }
        'o' => {
            let v = integers_unsigned::<usize>(s);
            if s.chars().nth(7).unwrap() == 'r' {
                let row = screen[v[0]].clone();
                screen[v[0]] = row[row.len() - v[1]..].to_vec();
                screen[v[0]].extend(&row[..row.len() - v[1]]);
            } else {
                for _ in 0..v[1] {
                    let mut temp = false;
                    for i in 0..screen.len() {
                        swap(&mut screen[i][v[0]], &mut temp);
                    }
                    swap(&mut screen[0][v[0]], &mut temp);
                }
            }
        }
        _ => panic!(),
    });

    println!(
        "{:#?}",
        screen
            .iter()
            .map(|v| v
                .iter()
                .map(|b| if *b { '#' } else { '.' })
                .collect::<String>())
            .collect_vec()
    );

    screen.into_iter().flatten().filter(|a| *a).count()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/08")));
    println!("{:?}", part2(include_str!("../../input/08")));
}
