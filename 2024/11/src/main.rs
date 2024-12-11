#![feature(test)]
extern crate test;
use std::{env, fs::read_to_string};
use rustc_hash::FxHashMap;
use util::*;

pub fn advent(filename: &str) -> (u64, u64) {
    let input =  read_to_string(filename).expect("could not read input file");
    let (mut part1, mut part2) = (0,0);

    let mut cache: FxHashMap<(u64, u32), u64> = FxHashMap::default();

    for num in extract_numbers::<u64>(&input) {
        part1 += length_after_blinks(num, 25, &mut cache);
        part2 += length_after_blinks(num, 75, &mut cache);
    }

    (part1, part2)
}

fn length_after_blinks(stone: u64, blinks: u32, cache: &mut FxHashMap<(u64, u32), u64>) -> u64 {
    if blinks == 0 { 1 }
    else if stone == 0 {
        length_after_blinks(1, blinks-1, cache)
    } else {
        let ndigits = stone.ilog10() + 1;
        if ndigits&1 == 0 { // even?
            // Cache this expensive path
            if let Some(cached) = cache.get(&(stone, blinks)) {
                return *cached;
            }
            let split = 10u64.pow(ndigits / 2);
            let len = length_after_blinks(stone / split, blinks - 1, cache)
                    + length_after_blinks(stone % split, blinks - 1, cache);
            cache.insert((stone, blinks), len);
            len
        } else {
            length_after_blinks(stone*2024, blinks-1, cache)
        }
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
    #[test] fn ex1() { assert_eq!(advent("example.txt").0, 55312) }
    #[test] fn final1() { assert_eq!(advent("input.txt").0, 203228) }
    #[test] fn ex2() { assert_eq!(advent("example.txt").1, 65601038650482) }
    #[test] fn final2() { assert_eq!(advent("input.txt").1, 240884656550923) }

    #[bench] fn bench_advent(b: &mut Bencher) { b.iter(|| advent("input.txt")); }
}

