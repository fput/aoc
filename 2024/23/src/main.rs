#![feature(test)]
extern crate test;
use std::{env, fs::read_to_string};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

type Num = usize;

pub fn advent(filename: &str) -> (Num, String) {
    let input =  read_to_string(filename).expect("could not read input file");
    let mut part1 = 0;

    // Build adjacency list
    let mut adj = FxHashMap::<String, FxHashSet<String>>::default();
    let mut links = Vec::new();

    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        links.push((a,b));
        adj.entry(a.into()).or_default().insert(b.into());
        adj.entry(b.into()).or_default().insert(a.into());
    }

    let mut seen = Vec::new();
    let triangle_seen = |s: &Vec<_>, a, b, c| s.contains(&(a,b,c)) || s.contains(&(a,c,b)) || s.contains(&(b,a,c)) || s.contains(&(b,c,a)) || s.contains(&(c,a,b)) || s.contains(&(c,b,a));

    // Part 1: Find all triangles (containing a computer starting with 't')
    for &(a,b) in &links {
        if !(a.starts_with('t') || b.starts_with('t')) {continue};
        for c in adj.get(a).unwrap() {
            if adj.get(b).unwrap().contains(c) && !triangle_seen(&seen,a,b,c) {
                seen.push((a,b,c));
                part1 += 1;
            }
        }
    }

    // Part 2: Greedy find max clique
    let mut max_clique = Vec::new();
    let mut clique = FxHashSet::default();

    for initial in adj.keys() {
        clique.clear();
        clique.insert(initial);
        for c in adj.get(initial).unwrap() {
            if clique.iter().all(|&d| adj.get(c).unwrap().contains(d)) {
                clique.insert(c);
            }
        }
        if clique.len() > max_clique.len() {
            max_clique = clique.drain().collect();
        }
    }

    max_clique.sort_unstable();
    let part2 = max_clique.iter().join(",");
    
    (part1, part2)
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
    #[test] fn ex1() { assert_eq!(advent("example.txt").0, 7) }
    #[test] fn final1() { assert_eq!(advent("input.txt").0, 1077) }
    #[test] fn ex2() { assert_eq!(advent("example.txt").1, "co,de,ka,ta") }
    #[test] fn final2() { assert_eq!(advent("input.txt").1, "bc,bf,do,dw,dx,ll,ol,qd,sc,ua,xc,yu,zt") }

    #[bench] fn bench_advent(b: &mut Bencher) { b.iter(|| advent("input.txt")); }
}

