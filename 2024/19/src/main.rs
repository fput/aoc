#![feature(test)]
extern crate test;
use std::{env, fs::read_to_string};

type Num = usize;

pub fn advent(filename: &str) -> (Num, Num) {
    let input =  read_to_string(filename).expect("could not read input file");
    let mut lines = input.lines();
    let available: Vec<_> = lines.next().unwrap().split(", ").collect();
    lines.next(); // skip empty line

    let (mut part1, mut part2) = (0,0);
    let mut cache = vec![usize::MAX; 100];
    for pattern in lines {
        cache.fill(usize::MAX);
        let c = num_combinations(&pattern, 0, &available, &mut cache);
        part1 += (c > 0) as usize;
        part2 += c;
    }

    (part1, part2)
}

fn num_combinations(target: &str, idx: usize, available: &[&str], cache: &mut [usize]) -> usize {
    if idx == target.len() {return 1}
    if cache[idx] != usize::MAX {return cache[idx]}
    let mut total = 0;
    for &p in available {
        if target[idx..].starts_with(p) {
            total += num_combinations(target, idx+p.len(), available, cache);
        }
    }
    cache[idx] = total;
    total
}

fn main() {
    let filename = env::args().nth(1).expect("No argument found");
    let (part1, part2) = advent(&filename);
    print!("{}\n{}\n", part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    #[test] fn example_input() { dbg!(advent("example.txt")); }
    #[test] fn final_input() { dbg!(advent("input.txt")); }
    #[test] fn ex1() { assert_eq!(advent("example.txt").0, 6) }
    #[test] fn final1() { assert_eq!(advent("input.txt").0, 258) }
    #[test] fn ex2() { assert_eq!(advent("example.txt").1, 16) }
    #[test] fn final2() { assert_eq!(advent("input.txt").1, 632423618484345) }

    #[bench] fn bench_advent(b: &mut Bencher) { b.iter(|| advent("input.txt")); }
}

