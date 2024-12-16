#![feature(test)]
extern crate test;
use std::{env, fs::read_to_string};
use rustc_hash::FxHashSet;
use util::*;

type Num = i32;

pub fn advent(filename: &str) -> (Num, Num) {
    let input =  read_to_string(filename).expect("could not read input file");
    let (map, moves) = input.split_once("\n\n").expect("cannot find map and moves");

    let mut map1 = Matrix::from_str(map); // Map for part 1
    let mut map2 = Matrix::new(2 * map1.width, map1.height, b'.'); // Map for part 2

    for y in 0..map1.height {
        for x in 0..map1.width {
            let (c1, c2) = (Point::new(2 * x, y), Point::new(2 * x + 1, y));
            match map1[Point::new(x, y)] {
                b'#' => { map2[c1] = b'#'; map2[c2] = b'#' }
                b'O' => { map2[c1] = b'['; map2[c2] = b']' }
                b'@' => { map2[c1] = b'@' }
                _ => {}
            };
        }
    }

    let mut pos1 = map1.find(b'@').expect("cannot find robot in map");
    let mut pos2 = Point::new(2 * pos1.x, pos1.y);

    let mut seen = FxHashSet::default();
    let mut buf = Vec::new();

    for dir_char in moves.lines().flat_map(str::chars) {
        let dir = match dir_char {
            '^' => UP,
            '<' => LEFT,
            'v' => DOWN,
            '>' => RIGHT,
            _ => unreachable!("invalid direction character")
        };

        move_robot(&mut map1, &mut pos1, dir, &mut buf, &mut seen);
        move_robot(&mut map2, &mut pos2, dir, &mut buf, &mut seen);
    }

    let part1 = map1.find_all(b'O').map(|b| 100 * b.y + b.x).sum::<i32>();
    let part2 = map2.find_all(b'[').map(|b| 100 * b.y + b.x).sum::<i32>();
    (part1, part2)
}

fn move_robot(map: &mut Matrix<u8>, pos: &mut Point, dir: Point, bfs: &mut Vec<Point>, seen: &mut FxHashSet<Point>) {
    seen.clear();
    bfs.clear(); bfs.push(*pos);

    let mut i = 0;
    while i < bfs.len() {
        let next = bfs[i] + dir; i+=1;

        match map[next] {
            b'#' => return, // blocked by a wall
            b'[' | b']' | b'O' => {
                // If we haven't seen this box yet, enqueue it.
                if seen.insert(next) {
                    bfs.push(next);
                    // For vertical pushes, if it's part of a two-tile box, enqueue its partner.
                    if dir.y != 0 && (map[next] == b'[' || map[next] == b']') {
                        let other = next + if map[next] == b'[' { RIGHT } else { LEFT };
                        if seen.insert(other) { bfs.push(other); }
                    }
                }
            }
            _ => continue // empty space
        }
    }

    // Move all boxes in reverse order to avoid overwriting
    for &tile in bfs.iter().rev() {
        (map[tile], map[tile + dir]) = (map[tile + dir], map[tile]);
    }
    *pos += dir;
    //println!("Moving {:?}. Robot now at {:?}", dir, pos);
    //map.print();
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
    #[test] fn ex1() { assert_eq!(advent("example.txt").0, 10092) }
    #[test] fn final1() { assert_eq!(advent("input.txt").0, 1406392) }
    #[test] fn ex2() { assert_eq!(advent("example.txt").1, 9021) }
    #[test] fn final2() { assert_eq!(advent("input.txt").1, 1429013) }

    #[bench] fn bench_advent(b: &mut Bencher) { b.iter(|| advent("input.txt")); }
}

