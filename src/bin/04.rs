use std::fs;

use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

fn check(s: &str) -> usize {
    let mut count = 0;
    let s = s.chars().collect::<Vec<_>>();
    for i in 0..s.len() {
        if i + 3 >= s.len() {
            break;
        }
        if s[i] == 'X' && s[i + 1] == 'M' && s[i + 2] == 'A' && s[i + 3] == 'S' {
            count += 1;
        }
        if s[i] == 'S' && s[i + 1] == 'A' && s[i + 2] == 'M' && s[i + 3] == 'X' {
            count += 1;
        }
    }

    count
}

fn part1() -> Result<()> {
    let input = fs::read_to_string("/home/jeremy/github/adventofrust/input/04")?;
    // let input = fs::read_to_string("/home/jeremy/github/adventofrust/input/test")?;

    let re = Regex::new("XMAS|SAMX").unwrap();

    let horz = input.lines().map(check).sum::<usize>();

    let lines = input.lines().collect::<Vec<_>>();
    let mut cols = Vec::new();
    for i in 0..lines[0].len() {
        let mut col_s = String::new();
        for line in &lines {
            col_s.push(line.chars().nth(i).unwrap());
        }
        cols.push(col_s);
    }

    let mut diag = Vec::new();

    for total in 0..2 * lines[0].len().max(lines.len()) {
        let mut s = String::new();
        for c_idx in 0..=total {
            let l_idx = total - c_idx;
            if c_idx >= lines[0].len() || l_idx >= lines.len() {
                continue;
            }

            s.push(lines[l_idx].chars().nth(c_idx).unwrap());
        }
        diag.push(s);
    }
    let lines = lines
        .into_iter()
        .map(|s| s.chars().rev().collect::<String>())
        .collect::<Vec<_>>();
    for total in 0..2 * lines[0].len().max(lines.len()) {
        let mut s = String::new();
        for c_idx in 0..=total {
            let l_idx = total - c_idx;
            if c_idx >= lines[0].len() || l_idx >= lines.len() {
                continue;
            }

            s.push(lines[l_idx].chars().nth(c_idx).unwrap());
        }
        diag.push(s);
    }

    let vert = cols.into_iter().map(|s| check(&s)).sum::<usize>();
    let diag = diag.into_iter().map(|s| check(&s)).sum::<usize>();

    let out = horz + vert + diag;

    println!("{out}");

    Ok(())
}

fn get_nine(g: &[Vec<char>], line: usize, char: usize) -> [[char; 3]; 3] {
    [
        [g[line][char], g[line][char + 1], g[line][char + 2]],
        [
            g[line + 1][char],
            g[line + 1][char + 1],
            g[line + 1][char + 2],
        ],
        [
            g[line + 2][char],
            g[line + 2][char + 1],
            g[line + 2][char + 2],
        ],
    ]
}

fn check_nine(nine: [[char; 3]; 3]) -> bool {
    if nine[1][1] != 'A' {
        false
    } else {
        let a = match (nine[0][0], nine[2][2]) {
            ('M', 'S') | ('S', 'M') => true,
            _ => false,
        };
        let b = match (nine[0][2], nine[2][0]) {
            ('M', 'S') | ('S', 'M') => true,
            _ => false,
        };
        a && b
    }
}

fn part2() -> Result<()> {
    let input = fs::read_to_string("/home/jeremy/github/adventofrust/input/04")?;

    let grid: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();

    let mut c = 0;
    for line in 0..grid.len() - 2 {
        for char in 0..grid[0].len() - 2 {
            let n = get_nine(&grid, line, char);
            if check_nine(n) {
                c += 1;
            }
        }
    }

    println!("{c}");

    Ok(())
}

fn main() {
    part1().unwrap();
    part2().unwrap();
}
