#![feature(test)]
extern crate test;
use std::{env, fs::read_to_string};
use itertools::Itertools;
use util::*;

// Expected number of lines (but also works if more lines)
const N: usize = 1000;

fn advent(filename: &str) -> (u32, u32) {
    let contents = read_to_string(filename)
        .expect("Something went wrong reading the input file");

    let (mut part1, mut part2) = (0, 0);
    let mut left: Vec<u32> = Vec::with_capacity(N);
    let mut right: Vec<u32> = Vec::with_capacity(N);

    for (a,b) in extract_numbers::<u32>(&contents).tuples() {
        // Destructure a line like
        //     123   456
        // into the variables a=123 and b=456
        left.push(a);
        right.push(b);
    }

    assert_eq!(left.len(), right.len());
    left.sort_unstable();
    right.sort_unstable();

    for i in 0..left.len() {
        part1 += left[i].abs_diff(right[i]);
        let occurrences = right.iter().filter(|&&x| x == left[i]).count() as u32;
        part2 += left[i] * occurrences;
    }

    (part1,part2)
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
    #[test] fn ex1() { assert_eq!(advent("example.txt").0, 11) }
    #[test] fn final1() { assert_eq!(advent("input.txt").0, 1666427) }
    #[test] fn ex2() { assert_eq!(advent("example.txt").1, 31) }
    #[test] fn final2() { assert_eq!(advent("input.txt").1, 24316233) }

    #[bench] fn bench_advent(b: &mut Bencher) { b.iter(|| advent("input.txt")); }
}

