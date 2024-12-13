#![feature(test)]
extern crate test;
use std::{env, fs::read_to_string};
use util::*;

type Num = i64;

pub fn advent(filename: &str) -> (Num, Num) {
    let input =  read_to_string(filename).expect("could not read input file");
    let lines = input.lines().collect::<Vec<_>>();
    let (mut part1, mut part2) = (0,0);

    let extract_pair = |l: &str| {
        let mut nums = extract_numbers::<Num>(l);
        (nums.next().unwrap(), nums.next().unwrap())
    };

    for config in lines.chunks(4) {
        let a = extract_pair(config[0]);
        let b = extract_pair(config[1]);
        let d = extract_pair(config[2]);
        let d2 = (d.0 + 10000000000000, d.1 + 10000000000000);

        part1 += tokens(a, b, d).unwrap_or(0);
        part2 += tokens(a, b, d2).unwrap_or(0);
    }

    (part1, part2)
}

/// Computes the integer coefficients `i` and `j` that solve the 2D vector
/// equation `i*a + j*b = d`.
///
/// The function attempts to find integers `i` and `j` such that the linear
/// combinations of vectors `a` and `b` result in vector `d`. Specifically,
/// it solves the equations:
/// 
///   `ax * i + bx * j = dx`
///   `ay * i + by * j = dy`
/// 
/// The function returns the amount of tokens `3*i + j`. Here:
///   `i` corresponds to the amount of A button presses, and
///   `j` corresponds to the amount of B button presses.
#[inline]
fn tokens(a: (Num, Num), b: (Num, Num), d: (Num, Num)) -> Option<Num> {
    let det = a.0*b.1 - a.1*b.0; // Determinant (should not be zero)
    let j = (d.1*a.0 - a.1*d.0) / det;
    let i = (d.0 - b.0*j) / a.0;

    if a.0*i + b.0*j == d.0 && a.1*i + b.1*j == d.1 {
        Some(3*i + j)
    } else {
        None
    }
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
    #[test] fn ex1() { assert_eq!(advent("example.txt").0, 480) }
    #[test] fn final1() { assert_eq!(advent("input.txt").0, 31065) }
    #[test] fn ex2() { assert_eq!(advent("example.txt").1, 875318608908) }
    #[test] fn final2() { assert_eq!(advent("input.txt").1, 93866170395343) }

    #[bench] fn bench_advent(b: &mut Bencher) { b.iter(|| advent("input.txt")); }
}

