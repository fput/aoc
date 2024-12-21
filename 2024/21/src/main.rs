#![feature(test)]
extern crate test;
use std::{env, fs::read_to_string};
use rustc_hash::FxHashMap;
use util::*;

type Num = usize;
const NUMPAD: &str = "789\n456\n123\n 0A";
const DIRPAD: &str = " ^A\n<v>";

pub fn advent(filename: &str) -> (Num, Num) {
    let input =  read_to_string(filename).expect("could not read input file");

    let numpad = Matrix::from_str(NUMPAD);
    let dirpad = Matrix::from_str(DIRPAD);

    let (mut part1, mut part2) = (0,0);
    let mut seq_cache = FxHashMap::default();

    for line in input.lines() {
        if line.is_empty() {break;}
        let numeric_part: usize = extract_numbers::<usize>(line).next().unwrap();

        let (mut seq_len1, mut seq_len2) = (0, 0);
        let mut prev_key = b'A';
        
        for key in line.bytes() {
            let mut prev_dir = b'A';
            for dir in seq_from_to(prev_key, key, &numpad) {
                seq_len1 += shortest_seq_len(prev_dir, dir, &dirpad, 2, &mut seq_cache);
                seq_len2 += shortest_seq_len(prev_dir, dir, &dirpad, 25, &mut seq_cache);
                prev_dir = dir;
            }
            prev_key = key;
        }
        part1 += numeric_part * seq_len1;
        part2 += numeric_part * seq_len2;
    }

    (part1, part2)
}

/// DP[from][to][layers]. The parameter [layers] holds the remaining number
/// of dirpad robot layers to consider. Memoization using [cache].
/// 
/// Returns the length of the shortest sequence after that many layers
/// from start to finish.
fn shortest_seq_len(from: u8, to: u8, pad: &Matrix<u8>, layers: u8, cache: &mut FxHashMap<(u8,u8,u8), usize>) -> usize {
    if let Some(result) = cache.get(&(from, to, layers)) {return *result}
    let mut result = 0;

    let seq = seq_from_to(from, to, &pad);
    if layers == 1 { result = seq.len() }
    else {
        let mut prev = b'A';
        for key in seq {
            result += shortest_seq_len(prev, key, pad, layers-1, cache);
            prev = key;
        }
    }
    cache.insert((from, to, layers), result);
    result
}

/// The optimal way to get from start to end is by minimizing the direction
/// changes along the path. So either move all the way horizontally first
/// and then vertically or the other way round (in case that would cross the
/// missing corner in the pad). Second, prefer going left (horizontal) first
/// if we have to go left. Otherwise, go vertical first.
fn seq_from_to(from: u8, to: u8, pad: &Matrix<u8>) -> Vec<u8> {
    let start = pad.find(from).expect("no start found");
    let end = pad.find(to).expect("no end found");
    
    let d = end-start;
    let vert = if d.y >=0 {b'v'} else {b'^'}; // vertical direction
    let hori = if d.x >=0 {b'>'} else {b'<'}; // horizontal direction

    let mut resulting_path = Vec::with_capacity(d.x.abs()as usize+d.y.abs()as usize);
    if (d.x < 0 && pad[Point::new(end.x, start.y)] != b' ')
       || pad[Point::new(start.x, end.y)] == b' ' {
        for _ in 0..d.x.abs() { resulting_path.push(hori) } // horizontal first
        for _ in 0..d.y.abs() { resulting_path.push(vert) }
    } else {
        for _ in 0..d.y.abs() { resulting_path.push(vert) } // vertical first
        for _ in 0..d.x.abs() { resulting_path.push(hori) }
    }
    resulting_path.push(b'A');
    resulting_path
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
    #[test] fn ex1() { assert_eq!(advent("example.txt").0, 126384) }
    #[test] fn final1() { assert_eq!(advent("input.txt").0, 188384) }
    #[test] fn ex2() { assert_eq!(advent("example.txt").1, 154115708116294) }
    #[test] fn final2() { assert_eq!(advent("input.txt").1, 232389969568832) }

    #[bench] fn bench_advent(b: &mut Bencher) { b.iter(|| advent("input.txt")); }
}

