#![feature(test)]
extern crate test;
use std::{env,fs::read};
use util::*;

const WIDTH: usize = 130; // maximum width
const HEIGHT: usize = WIDTH;
type Generation = u16;

pub fn advent(filename: &str) -> (u32, u32) {
    let mut map =  Matrix::from_bytes(&read(filename).expect("could not read input file"));
    assert_eq!(map.height, map.width);
    assert!(WIDTH>=map.width as usize);
    
    let mut pos = map.find(b'^').expect("no start position found");
    let mut dir = Direction::Up;
    let mut gen: Generation = 0;

    // This array keeps track of map positions + directions that have been visited before.
    // Initialize all to zero. We will store generation numbers here, which get incremented for each
    // separate cycle check.
    let mut visited: [Generation; WIDTH * HEIGHT * 4] = [gen; WIDTH * HEIGHT * 4];

    let (mut part1, mut part2) = (1,0);

    while let Some(next_cell) = map.get(pos + dir.to_point()) {
        match next_cell {
            b'#' => dir = dir.clockwise(),
            b'.' => {
                let next = pos + dir.to_point();
                part1 += 1;

                map[next] = b'#';
                gen+=1;
                if is_cyclic(&map, &mut visited, gen, pos, dir) {
                    part2 += 1;
                }
                map[next] = b'^';
                pos = next;
            },
            b'^' => pos = pos + dir.to_point(),
            _ => break,
        }
    }
    
    (part1, part2)
}

// Check if continuing on this path results in a loop.
// We have a loop if we visit a previosuly visited tile.
#[inline]
fn is_cyclic(map: &Matrix<u8>, visited: &mut [Generation; WIDTH * HEIGHT * 4], gen: Generation, mut pos: Point, mut dir: Direction) -> bool {
    while let Some(next_cell) = map.get(pos+dir.to_point()) {
        if *next_cell == b'#' {
            let state_idx = ((pos.x as usize) * WIDTH + (pos.y as usize)) * 4 + dir.to_index();
            if visited[state_idx] == gen {
                return true;
            }
            visited[state_idx] = gen;
            dir = dir.clockwise();
            continue;
        }

        pos = pos + dir.to_point();
    }

    false
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
    #[test] fn ex1() { assert_eq!(advent("example.txt").0, 41) }
    #[test] fn final1() { assert_eq!(advent("input.txt").0, 4665) }
    #[test] fn ex2() { assert_eq!(advent("example.txt").1, 6) }
    #[test] fn final2() { assert_eq!(advent("input.txt").1, 1688) }

    #[bench] fn bench_advent(b: &mut Bencher) { b.iter(|| advent("input.txt")); }
}

