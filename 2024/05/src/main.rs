#![feature(test)]
extern crate test;
use std::{env,fs::read_to_string,cmp::Ordering};
use itertools::Itertools;
use util::*;

const MAX_NUMBER: usize = 99;

fn advent(filename: &str) -> (u32, u32) {
    let input = read_to_string(filename).expect("could not read input file");
    let (rules, updates) = input.split_once("\n\n").expect("invalid input format");
    let (mut part1, mut part2) = (0, 0);

    // The 2D array [page_before] defines the order of the pages.
    // page_before[A][B] is true if page A should be printed before page B.
    // Otherwise it is false.
    //
    // The input seems to contain N different numbers between 11 and 99.
    // To define the order between every pair we need a rule for every subset of size 2.
    // For the example input, we have 7 different numbers, and indeed have C(7,2) = 21 rules.
    // As every rule is specified, we can safely assign a default value.
    // 
    // The rule format does not allow for equal order, therefore there are only two
    // possible orders (before/after).
    let mut page_before = [[false; MAX_NUMBER]; MAX_NUMBER];
    for (a,b) in extract_numbers::<usize>(&rules).tuples() {
        page_before[a][b] = true; // the other direction [b][a] is false by default
    }

    let mut update_buf = Vec::new();
    for update_line in updates.lines() {
        // Reuse a vector to avoid repeated memory allocations (~30 µs)
        update_buf.clear();
        update_buf.extend(extract_numbers::<usize>(&update_line));

        if update_buf.is_sorted_by(|&a, &b| page_before[a][b]) {
            part1 += update_buf[update_buf.len() / 2];
        } else {
            update_buf.sort_unstable_by(|&a, &b| if page_before[a][b] {Ordering::Less} else {Ordering::Greater});
            part2 += update_buf[update_buf.len() / 2];
        }
    }

    (part1 as u32, part2 as u32)
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
    #[test] fn ex1() { assert_eq!(advent("example.txt").0, 143) }
    #[test] fn final1() { assert_eq!(advent("input.txt").0, 5248) }
    #[test] fn ex2() { assert_eq!(advent("example.txt").1, 123) }
    #[test] fn final2() { assert_eq!(advent("input.txt").1, 4507) }

    #[bench] fn bench_advent(b: &mut Bencher) { b.iter(|| advent("input.txt")); }
}

