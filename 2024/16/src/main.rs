#![feature(test)]
extern crate test;
use std::{collections::{BinaryHeap, VecDeque}, env, fs::read};
use rustc_hash::FxHashSet;
use util::*;

type Num = usize;

pub fn advent(filename: &str) -> (Num, Num) {
    let input =  read(filename).expect("could not read input file");
    let map = Matrix::from_bytes(&input);
    let start = map.find(b'S').expect("no start position");
    let mut scores = Matrix::new(map.width, map.height, [Num::MAX; 4]);

    // Part 1: Dijkstra to find path with minimum score.
    // Using a minheap, check all tiles in order of minimum score to construct a
    // minimum score path to the destination.
    let mut todo = BinaryHeap::new();
    todo.push(MinHeapEntry {key: 0, data: (start, Direction::Right) });
    scores[start][Direction::Right.to_index()] = 0;

    let mut part1 = Num::MAX;
    while let Some(MinHeapEntry{ key: score, data: (pos, dir)}) = todo.pop() {
        if map[pos] == b'E' { part1 = score; break; } 
        if score > scores[pos][dir.to_index()] || map[pos] == b'#' { continue; }

        // Queue the different movement options
        let forward = (pos + dir.to_point(), dir,               score + 1);
        let right =   (pos,                  dir.clockwise(),   score + 1000);
        let left =    (pos,                  dir.counterwise(), score + 1000);

        for &(npos, ndir, nscore) in &[forward, right, left] {
            // Only queue this movement if it doesn't move into a wall or if it is a more costly path
            if map[npos] != b'#' && nscore < scores[npos][ndir.to_index()] {
                scores[npos][ndir.to_index()] = nscore;
                todo.push(MinHeapEntry{key: nscore, data: (npos, ndir)});
            }
        }
    }

    // Part 2: BFS backwards from the end, marking minimal-path cells
    // based on whether the path matches the calculated scores in `scores`.
    let end = map.find(b'E').expect("no end position");
    let mut seen = FxHashSet::default();
    let mut todo = VecDeque::new();
    for dir in Direction::VALUES {
        if scores[end][dir.to_index()] == part1 {
            todo.push_back((end, dir));
            seen.insert(end);
        }
    }

    while let Some((pos, dir)) = todo.pop_front() {
        let score = scores[pos][dir.to_index()];
        let back_forward = (pos - dir.to_point(), dir,               1);
        let back_right =   (pos,                  dir.clockwise(),   1000);
        let back_left =    (pos,                  dir.counterwise(), 1000);

        for &(npos, ndir, diff) in &[back_forward, back_right, back_left] {
            if diff <= score && scores[npos][ndir.to_index()] == score - diff {
                todo.push_back((npos, ndir));
                seen.insert(npos);
            }
        }
    }

    (part1, seen.len())
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
    #[test] fn ex1() { assert_eq!(advent("example.txt").0, 7036) }
    #[test] fn ex12() { assert_eq!(advent("example2.txt").0, 11048) }
    #[test] fn final1() { assert_eq!(advent("input.txt").0, 73404) }
    #[test] fn ex2() { assert_eq!(advent("example.txt").1, 45) }
    #[test] fn ex22() { assert_eq!(advent("example2.txt").1, 64) }
    #[test] fn final2() { assert_eq!(advent("input.txt").1, 449) }

    #[bench] fn bench_advent(b: &mut Bencher) { b.iter(|| advent("input.txt")); }
}

