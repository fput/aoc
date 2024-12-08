#![feature(test)]
extern crate test;
use std::{env, fs::read};
use rustc_hash::FxHashMap;
use util::*;

pub fn advent(filename: &str) -> (u32, u32) {
    let map =  Matrix::from_bytes(&read(filename).expect("could not read input file"));
    assert_eq!(map.height, map.width);

    // Map to store antinodes with a '#'
    let mut antinodes1 = map.clone(); // part1
    let mut antinodes2 = map.clone(); // part2
    let mut antenna_positions: FxHashMap<u8, Vec<Point>> = FxHashMap::default();
    let (mut part1, mut part2) = (0,0);

    for x in 0..map.width {
        for y in 0..map.height {
            let p1 = Point::new(x,y);
            if map[p1] == b'.' { continue; }

            if let Some(positions) = antenna_positions.get(&map[p1]) {
                for &p2 in positions {
                    update_antinodes(&mut antinodes1, &mut antinodes2, p1, p2, &mut part1, &mut part2);
                    update_antinodes(&mut antinodes1, &mut antinodes2, p2, p1, &mut part1, &mut part2);
                }
            }
            antenna_positions.entry(map[p1]).or_default().push(p1);
        }
    }

    (part1, part2)
}

// Find and count all antinodes for the given antenna positions.
fn update_antinodes(antinodes1: &mut Matrix<u8>, antinodes2: &mut Matrix<u8>, p1: Point, p2: Point, part1: &mut u32, part2: &mut u32) {
    let dp = p2 - p1;
    for i in 0..antinodes1.width {
        let pos = p1 + dp*i;
        if !antinodes1.contains(pos) { break; }
        if i == 2 {
            *part1 += if antinodes1.update(pos, b'#') {1} else {0};
        }
        *part2 += if antinodes2.update(pos, b'#') {1} else {0}
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
    #[test] fn ex1() { assert_eq!(advent("example.txt").0, 14) }
    #[test] fn final1() { assert_eq!(advent("input.txt").0, 398) }
    #[test] fn ex2() { assert_eq!(advent("example.txt").1, 34) }
    #[test] fn final2() { assert_eq!(advent("input.txt").1, 1333) }

    #[bench] fn bench_advent(b: &mut Bencher) { b.iter(|| advent("input.txt")); }
}

