#![feature(test)]
extern crate test;
use std::{env,fs::read_to_string};
use itertools::Itertools;
use util::*;

fn advent(filename: &str) -> (u32, u32) {
    let contents = read_to_string(filename)
        .expect("Something went wrong reading the input file");
    let (mut part1, mut part2) = (0,0);

    for line in contents.lines() {
        let report = extract_numbers::<i32>(line).collect_vec();

        // Part 1: Check if the levels are safe
        if is_safe_report(report.iter().cloned()) {
            part1 += 1;
        }

        // Part 2: Check if the levels are safe, including when removing one level
        for i in 0..report.len() {
            // Create an iterator that skips over one level
            let skipped_iter = report.iter().enumerate()
                .filter_map(|(idx, &val)| if idx != i { Some(val) } else { None });

            if is_safe_report(skipped_iter) {
                part2 += 1;
                break; // Break once we find one level to remove safely
            }
        }
    }

    (part1, part2)
}

// Checks whether a report contains only safe levels.
// 
// The argument is an iterator over the levels to consider.
// 
// A report counts as safe if both of the following are true:
//  - The levels are either all increasing or all decreasing.
//  - Any two adjacent levels differ by at least one and at most three.
fn is_safe_report<I>(iter: I) -> bool
where
    I: Iterator<Item = i32> + Clone,
{
    // Differences between adjacent levels in the report
    let mut diffs = iter.tuple_windows().map(|(x, y)| y - x);
    // Check the safety condition
    diffs.clone().all(|x| x >=  1 && x <=  3)
         || diffs.all(|x| x >= -3 && x <= -1)
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
    #[test] fn ex1() { assert_eq!(advent("example.txt").0, 2) }
    #[test] fn final1() { assert_eq!(advent("input.txt").0, 591) }
    #[test] fn ex2() { assert_eq!(advent("example.txt").1, 4) }
    #[test] fn final2() { assert_eq!(advent("input.txt").1, 621) }

    #[bench] fn bench_advent(b: &mut Bencher) { b.iter(|| advent("input.txt")); }
}

