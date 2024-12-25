#![feature(test)]
extern crate test;
use std::{env, fs::read_to_string};
use util::*;

type Num = i32;
const HEIGHT: Num = 6;
const WIDTH: Num = 5;
const TOP: Point = Point::new(0,0);

pub fn advent(filename: &str) -> (Num, Num) {
    let input = read_to_string(filename).expect("could not read input file");
    let mut keys = Vec::with_capacity(250);
    let mut locks = Vec::with_capacity(250);

    let (mut part1, part2) = (0,0);

    for schematic in input.split("\n\n") {
        let mat = Matrix::from_str(schematic);
        let mut heights = [0; 5];
        for x in 0..WIDTH {
            for y in 1..HEIGHT { // count number of # in all cols except top/bottom
                if mat[Point::new(x,y)] == b'#' {
                    heights[x as usize] += 1;
                }
            }
        }
        if mat[TOP] == b'#' { // Lock
            locks.push(heights);
        } else { // Key
            keys.push(heights);
        }
    }

    // Try all lock combinations
    for key in &keys {
        'LOCKS: for lock in &locks {
            for x in 0..WIDTH {
                if key[x as usize] + lock[x as usize] > 5 { continue 'LOCKS }
            }
            part1 += 1;
        }
    }
    
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
    #[test] fn ex1() { assert_eq!(advent("example.txt").0, 3) }
    #[test] fn final1() { assert_eq!(advent("input.txt").0, 3077) }

    #[bench] fn bench_advent(b: &mut Bencher) { b.iter(|| advent("input.txt")); }
}

