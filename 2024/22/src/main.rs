#![feature(test)]
extern crate test;
use std::{collections::VecDeque, env, fs::read_to_string};

type Num = usize;
const ITERATIONS: usize = 2000;
const DIFF_RANGE: usize = 19;

pub fn advent(filename: &str) -> (Num, Num) {
    let input =  read_to_string(filename).expect("could not read input file");
    let mut part1 = 0;
    let mut total = vec![vec![vec![vec![0; DIFF_RANGE]; DIFF_RANGE]; DIFF_RANGE]; DIFF_RANGE];
    let mut seen = vec![vec![vec![vec![Num::MAX; DIFF_RANGE]; DIFF_RANGE]; DIFF_RANGE]; DIFF_RANGE];
    let mut seq = VecDeque::with_capacity(4); // stores the last four sequence elements (= diff)

    for (lineno,line) in input.lines().enumerate() {
        let mut secret = line.parse::<Num>().unwrap();
        let mut price = 0;
        for i in 0..ITERATIONS {
            secret = next_secret_number(secret);
            let next_price = secret % 10;
            if seq.len() == seq.capacity() { seq.pop_front(); } // only keep last four
            seq.push_back(next_price + 9 - price);
            price = next_price;

            if i >= 3 {
                let idx = (seq[0], seq[1], seq[2], seq[3]);
                if  seen[idx.0][idx.1][idx.2][idx.3] != lineno {
                    seen[idx.0][idx.1][idx.2][idx.3]  = lineno;
                    total[idx.0][idx.1][idx.2][idx.3] += price;
                }
            }
        }
        part1 += secret;
    }
   
    let part2 = *total.iter().flatten().flatten().flatten().max().unwrap();
    
    (part1, part2)
}

#[inline]
fn next_secret_number(mut num: Num) -> Num {
    num = num ^ (num << 6) & 0xFFFFFF;
    num = num ^ (num >> 5) & 0xFFFFFF;
          num ^ (num << 11) & 0xFFFFFF
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
    #[test] fn ex1() { assert_eq!(advent("example.txt").0, 37327623) }
    #[test] fn final1() { assert_eq!(advent("input.txt").0, 20332089158) }
    #[test] fn ex2() { assert_eq!(advent("example2.txt").1, 23) }
    #[test] fn final2() { assert_eq!(advent("input.txt").1, 2191) }

    #[bench] fn bench_advent(b: &mut Bencher) { b.iter(|| advent("input.txt")); }
}

