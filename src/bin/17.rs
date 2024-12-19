#![feature(let_chains)]

use std::collections::*;

use itertools::Itertools;
use num::pow::Pow;
use regex::Regex;

use adventofrust::utils::*;

fn get_combo(op: usize, regs: &[usize]) -> usize {
    match op {
        0..=3 => op,
        4 => regs[0],
        5 => regs[1],
        6 => regs[2],
        _ => panic!(),
    }
}

fn part1() {
    let input = include_str!("/home/jeremy/github/adventofrust/input/17");

    let mut registers = integers_unsigned::<usize>(input)
        .into_iter()
        .take(3)
        .collect_vec();

    let insts = integers_unsigned::<usize>(input)
        .into_iter()
        .skip(3)
        .collect_vec();

    let mut pc = 0;

    let mut out: Vec<usize> = Vec::new();

    while let Some(instruction) = insts.get(pc) {
        match instruction {
            0 => registers[0] /= 2usize.pow(get_combo(insts[pc + 1], &registers) as u32),
            1 => registers[1] ^= insts[pc + 1],
            2 => registers[1] = get_combo(insts[pc + 1], &registers) & 0b111,
            3 => {
                if registers[0] > 0 {
                    pc = insts[pc + 1];
                    continue;
                }
            }
            4 => registers[1] ^= registers[2],
            5 => out.push(get_combo(insts[pc + 1], &registers) & 0b111),
            6 => {
                registers[1] =
                    registers[0] / 2usize.pow(get_combo(insts[pc + 1], &registers) as u32)
            }
            7 => {
                registers[2] =
                    registers[0] / 2usize.pow(get_combo(insts[pc + 1], &registers) as u32)
            }
            _ => panic!(),
        }
        pc += 2;
    }

    println!("{}", out.into_iter().join(","));
}

fn check(mut registers: [usize; 3], insts: &[usize], max_depth: usize) -> Vec<usize> {
    let mut pc = 0;

    let mut out: Vec<usize> = Vec::new();

    let mut depth = 0;

    while let Some(instruction) = insts.get(pc) {
        match instruction {
            0 => registers[0] /= 2usize.pow(get_combo(insts[pc + 1], &registers) as u32),
            1 => registers[1] ^= insts[pc + 1],
            2 => registers[1] = get_combo(insts[pc + 1], &registers) & 0b111,
            3 => {
                if registers[0] > 0 {
                    pc = insts[pc + 1];
                    continue;
                }
            }
            4 => registers[1] ^= registers[2],
            5 => {
                out.push(get_combo(insts[pc + 1], &registers) & 0b111);
                depth += 1;
                if depth == max_depth {
                    break;
                }
            }
            6 => {
                registers[1] =
                    registers[0] / 2usize.pow(get_combo(insts[pc + 1], &registers) as u32)
            }
            7 => {
                registers[2] =
                    registers[0] / 2usize.pow(get_combo(insts[pc + 1], &registers) as u32)
            }
            _ => panic!(),
        }
        pc += 2;
    }

    out
}

fn part2() {
    let input = include_str!("/home/jeremy/github/adventofrust/input/17");

    let registers = integers_unsigned::<usize>(input)
        .into_iter()
        .take(3)
        .collect_vec();

    let insts = integers_unsigned::<usize>(input)
        .into_iter()
        .skip(3)
        .collect_vec();

    let mut nums = vec![0];
    for d in (0..insts.len()).rev() {
        let mut new_nums = Vec::new();
        for num in nums {
            new_nums.extend(
                (0..8)
                    .filter(|a| {
                        check([num << 3 | a, registers[1], registers[2]], &insts, 1)
                            == insts[d..d + 1]
                    })
                    .map(|a| num << 3 | a),
            );
        }
        nums = new_nums;
    }

    println!("{}", nums.into_iter().min().unwrap());
}

fn main() {
    part1();
    part2();
}
