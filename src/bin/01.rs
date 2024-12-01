use std::fs;

use anyhow::Result;

fn part1() -> Result<()> {
    let input = fs::read_to_string("/home/jeremy/github/adventofrust/input/01")?;

    let mut a = Vec::new();
    let mut b = Vec::new();

    for line in input.lines() {
        let mut vals = line.split("   ");
        a.push(vals.next().unwrap());
        b.push(vals.next().unwrap());
    }

    a.sort();
    b.sort();

    let out = a
        .iter()
        .zip(b)
        .map(|(a, b)| {
            a.parse::<usize>()
                .unwrap()
                .abs_diff(b.parse::<usize>().unwrap())
        })
        .sum::<usize>();

    println!("{out}");

    Ok(())
}

fn part2() -> Result<()> {
    let input = fs::read_to_string("/home/jeremy/github/adventofrust/input/01")?;

    let mut a = Vec::new();
    let mut b = Vec::new();

    for line in input.lines() {
        let mut vals = line.split("   ");
        a.push(vals.next().unwrap());
        b.push(vals.next().unwrap());
    }

    let out = a
        .iter()
        .map(|a| b.iter().filter(|b| *b == a).count() * a.parse::<usize>().unwrap())
        .sum::<usize>();

    println!("{out}");

    Ok(())
}

fn main() {
    part1().unwrap();
    part2().unwrap();
}
