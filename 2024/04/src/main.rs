// Optimized solution using one dimensional byte vector for a constant HEIGHT x WIDTH.
// Includes a fallback to the general solution in the file main_vector.rs

#![feature(test)]
extern crate test;
use std::{env,fs::read};
use util::*;

const MAS: [u8; 3] = [b'M', b'A', b'S'];
const SAM: [u8; 3] = [b'S', b'A', b'M'];

fn advent(filename: &str) -> (u32, u32) {
    let map = Matrix::from_bytes(&read(filename).expect("could not read input file"));
    let (mut part1, mut part2) = (0,0);

    for r in 0..map.height {
        for c in 0..map.width {
            let p = Point::new(r,c);
            // Summing directions for part1
            for &dir in &[LEFT, RIGHT, UP, DOWN, NORTHEAST, NORTHWEST, SOUTHEAST, SOUTHWEST] {
                part1 += direction_contains_word(&map, p, dir, "XMAS") as u32;
            }
            // Check mas function for part2
            part2 += mas_pattern(&map, p) as u32;
        }
    }

    (part1, part2)
}

fn direction_contains_word(grid: &Matrix<u8>, p: Point, dir: Point, word: &str) -> bool {
    let mut chars = word.chars();
    (0..word.len() as i32).all(|i| {
        grid.get(p + dir*i).is_some_and(|&c| c == chars.next().unwrap() as u8)
    })
}

fn mas_pattern(grid: &Matrix<u8>, p: Point) -> bool {
    if !(grid.contains(p+SOUTHEAST) && grid.contains(p+NORTHWEST)) {return false;}

    let forward = [
        grid[p+SOUTHWEST],
        grid[p],
        grid[p+NORTHEAST],
    ];
    let backward = [
        grid[p+NORTHWEST],
        grid[p],
        grid[p+SOUTHEAST],
    ];
    (forward == MAS || forward == SAM) && (backward == MAS || backward == SAM)
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
    #[test] fn ex1() { assert_eq!(advent("example.txt").0, 18) }
    #[test] fn final1() { assert_eq!(advent("input.txt").0, 2646) }
    #[test] fn ex2() { assert_eq!(advent("example.txt").1, 9) }
    #[test] fn final2() { assert_eq!(advent("input.txt").1, 2000) }

    #[bench] fn bench_advent(b: &mut Bencher) { b.iter(|| advent("input.txt")); }
}

