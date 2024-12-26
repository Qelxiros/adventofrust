use std::fmt::Debug;

fn part1() -> impl Debug {
    let input = include_str!("../../input/01");

    let mut a = Vec::new();
    let mut b = Vec::new();

    for line in input.lines() {
        let mut vals = line.split("   ");
        a.push(vals.next().unwrap());
        b.push(vals.next().unwrap());
    }

    a.sort();
    b.sort();

    a.iter()
        .zip(b)
        .map(|(a, b)| {
            a.parse::<usize>()
                .unwrap()
                .abs_diff(b.parse::<usize>().unwrap())
        })
        .sum::<usize>()
}

fn part2() -> impl Debug {
    let input = include_str!("../../input/01");

    let mut a = Vec::new();
    let mut b = Vec::new();

    for line in input.lines() {
        let mut vals = line.split("   ");
        a.push(vals.next().unwrap());
        b.push(vals.next().unwrap());
    }

    a.iter()
        .map(|a| {
            b.iter().filter(|b| *b == a).count() * a.parse::<usize>().unwrap()
        })
        .sum::<usize>()
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}
