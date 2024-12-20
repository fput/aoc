#![feature(test)]
extern crate test;
use std::{collections::VecDeque, env, fs::read};
use util::*;

type Num = usize;
const MIN_PICO: usize = 100;

pub fn advent(filename: &str) -> (Num, Num) {
    let input =  read(filename).expect("could not read input file");
    let map = Matrix::from_bytes(&input);
    let start = map.find(b'S').expect("no start position");
    let end = map.find(b'E').expect("no end position");
    let mut picos = Matrix::new(map.width, map.height, Num::MAX);
    picos[start] = 0;
    let mut next_pico = 1;

    // BFS to find path from start to end
    let mut todo = VecDeque::new();
    todo.push_back(start);
    let mut path = vec![start];
    while let Some(pos) = todo.pop_front() {
        if pos == end { break; } 
        for &npos in &[pos + LEFT, pos + RIGHT, pos + UP, pos + DOWN] {
            if map.contains(npos) && map[npos] != b'#' && picos[npos] > next_pico {
                picos[npos] = next_pico;
                todo.push_back(npos);
                path.push(npos);
            }
        }
        next_pico+=1;
    }

    let (mut part1, mut part2) = (0,0);

    // Check every pair in path: If distance is smaller than 20, it's a potential cheat.
    let total_time = path.len();
    for i in 0..total_time.saturating_sub(MIN_PICO) {
        let start = path[i];
        for j in (i+MIN_PICO)..total_time {
            let end = path[j];
            let cheat_time = (start.x-end.x).abs() + (start.y-end.y).abs();
            if cheat_time > 20 {continue;}
            let old_time = (j-i) as i32;
            if old_time - cheat_time >= MIN_PICO as i32 {
                part2 += 1;
                if cheat_time <= 2 {
                    part1 += 1;
                }
            }
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
    #[test] fn ex1() { assert_eq!(advent("example.txt").0, 0) }
    #[test] fn final1() { assert_eq!(advent("input.txt").0, 1338) }
    #[test] fn ex2() { assert_eq!(advent("example.txt").1, 0) }
    #[test] fn final2() { assert_eq!(advent("input.txt").1, 975376) }

    #[bench] fn bench_advent(b: &mut Bencher) { b.iter(|| advent("input.txt")); }
}

