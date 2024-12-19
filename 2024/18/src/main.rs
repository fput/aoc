#![feature(test)]
extern crate test;
use std::{collections::VecDeque, env, fs::read_to_string};
use util::*;

type Num = i32;

pub fn advent(filename: &str, width: usize, part1_bytes: usize) -> (Num, String) {
    let input =  read_to_string(filename).expect("could not read input file");
    let mut map = Matrix::new(width as i32, width as i32, b'.');

    let extract_pair = |l: &str| {
        let mut nums = extract_numbers::<Num>(l);
        (nums.next().unwrap(), nums.next().unwrap())
    };

    let mut part1 = 0;
    let mut part2: String = String::new();

    let mut todo = VecDeque::new();
    let mut seen = Matrix::new(map.width, map.height, Num::MAX);
    let mut current_min_length = 0;


    for (i, line) in input.lines().enumerate() {
        let (x,y) = extract_pair(line);
        let pos = Point::new(x,y);
        map[pos] = b'#';

        if i == part1_bytes - 1 {
            // Part 1: Shortest path length after given number of obstacles
            let start = Point::new(0, 0);
            todo.push_back(start);
            seen[start] = 0;
            part1 = shortest_path(&mut map, &mut todo, &mut seen).expect("end must be reachable for part 1");
            current_min_length = part1;
        } else if i >= part1_bytes {
            // Part 2: Find after which new obstacle there is no possible path anymore
            let obstacle_step = seen[pos];
            // If the obstacle is on an unreachable tile, it won't harm
            if obstacle_step == Num::MAX { continue; }
            // If the obstacle is somewhere after having reached the end, it won't harm
            if obstacle_step >= current_min_length { continue; }

            // Reset all tiles with higher step count than obstacle (so they can be explored again)
            for steps in seen.elements.iter_mut() {
                if *steps > obstacle_step {
                    *steps = Num::MAX;
                }
            }

            // Continue search from all tiles with same step count as obstacle.
            todo.clear();
            let mut alternative_route_available = false;
            for npos in seen.find_all(obstacle_step) {
                if npos != pos {
                    alternative_route_available = true;
                    todo.push_back(npos);
                }
            }
            // If there are no other tiles with the same step counts, this obstacle blocks the only path.
            if !alternative_route_available {
                part2 = format!("{x},{y}");
                break;
            }
            // See if there is a route remaining
            if let Some(result) = shortest_path(&mut map, &mut todo, &mut seen) {
                current_min_length = result;
            } else {
                part2 = format!("{x},{y}");
                break;
            }
        }
    }
    (part1, part2)
}

/// Compute shortest paths from top-left to all reachable cells using BFS.
/// Returns shortest path steps to the bottom-right if reachable.
fn shortest_path(map: &mut Matrix<u8>, todo: &mut VecDeque<Point>, seen: &mut Matrix<Num>) -> Option<Num> {
    let end = Point::new(map.width - 1, map.height - 1);
    let mut minlength = None;

    while let Some(pos) = todo.pop_front() {
        let steps = seen[pos];
        if pos == end && minlength.is_none() { minlength = Some(steps); }
        if map[pos] == b'#' { continue; }

        for &npos in &[pos + LEFT, pos + RIGHT, pos + UP, pos + DOWN] {
            let nsteps = steps + 1;
            if map.contains(npos) && map[npos] != b'#' && nsteps < seen[npos] {
                seen[npos] = nsteps;
                todo.push_back(npos);
            }
        }
    }
    minlength
}

fn main() {
    let filename = env::args().nth(1).expect("No argument found");
    let (part1, part2) = advent(&filename, 71, 1024);
    print!("{}\n{}\n", part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    #[test] fn example_input() { dbg!(advent("example.txt", 7, 12)); }
    #[test] fn final_input() { dbg!(advent("input.txt", 71, 1024)); }
    #[test] fn ex1() { assert_eq!(advent("example.txt", 7, 12).0, 22) }
    #[test] fn final1() { assert_eq!(advent("input.txt", 71, 1024).0, 246) }
    #[test] fn ex2() { assert_eq!(advent("example.txt", 7, 12).1, "6,1") }
    #[test] fn final2() { assert_eq!(advent("input.txt", 71, 1024).1, "22,50") }

    #[bench] fn bench_advent(b: &mut Bencher) { b.iter(|| advent("input.txt", 71, 1024)); }
}

