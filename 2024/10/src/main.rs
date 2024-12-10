#![feature(test)]
#![feature(iter_collect_into)]
extern crate test;
use std::{env, fs::read};
use rustc_hash::FxHashSet;
use util::*;

pub fn advent(filename: &str) -> (usize, usize) {
    let map =  Matrix::from_bytes_as_digits(&read(filename).expect("could not read input file"));
    let (mut part1, mut part2) = (0,0);
    let mut trailends: Vec<Point> = Vec::new();
    let mut unique_trailends: FxHashSet<Point> = FxHashSet::default();

    // Look backwards from 9->0 as my map contains less `9` than `0`
    for trailhead in map.find_all(9) {
        // Collects distinct trailends in `trailends`.
        distinct_trails(trailhead, 9, &map, &mut trailends);
        // Part 1: Unique trails
        part1 += trailends.iter().collect_into::<FxHashSet<Point>>(&mut unique_trailends).len();
        // Part 2: Total trails
        part2 += trailends.len();
        // Clear buffers for next iteration
        trailends.clear();
        unique_trailends.clear();
    }

    (part1, part2)
}

// Collects distinct trailends in `trailends`.
fn distinct_trails<'a>(start: Point, height: u8, map: &Matrix<u8>, trailends: &'a mut Vec<Point>) -> &'a mut Vec<Point> {
    for neighbor in [start+LEFT, start+RIGHT, start+UP, start+DOWN] {
        let target_height = height-1;
        if map.contains(neighbor) && map[neighbor] == target_height {
            if target_height == 0 {
                trailends.push(neighbor);
            } else {
                distinct_trails(neighbor, target_height, map, trailends);
            }
        }
    }
    trailends
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
    #[test] fn ex1() { assert_eq!(advent("example.txt").0, 36) }
    #[test] fn final1() { assert_eq!(advent("input.txt").0, 472) }
    #[test] fn ex2() { assert_eq!(advent("example.txt").1, 81) }
    #[test] fn final2() { assert_eq!(advent("input.txt").1, 969) }

    #[bench] fn bench_advent(b: &mut Bencher) { b.iter(|| advent("input.txt")); }
}

