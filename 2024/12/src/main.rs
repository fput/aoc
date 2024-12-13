#![feature(test)]
extern crate test;
use std::{collections::VecDeque, env, fs::read};
use util::*;

pub fn advent(filename: &str) -> (usize, usize) {
    let map =  Matrix::from_bytes_as_digits(&read(filename).expect("could not read input file"));
    let mut seen = Matrix::new(map.width, map.height, false);
    let (mut part1, mut part2) = (0,0);
    let mut current_region: VecDeque<Point> = VecDeque::new();

    for y in 0..map.height {
        for x in 0..map.width {
            let cur = Point::new(x, y);
            if seen[cur] { continue; }
            seen[cur] = true;

            let (mut area, mut perimeter, mut sides) = (0, 0, 0);
            let plant_type = map[cur];
            current_region.push_front(Point::new(x, y));

            while let Some(cur) = current_region.pop_front() {
                area += 1;
                for dir in [LEFT, RIGHT, UP, DOWN] {
                    let neighbor = cur + dir;
                    if map.get(neighbor) == Some(&plant_type) {
                        if !seen[neighbor] {
                            current_region.push_back(neighbor);
                            seen[neighbor] = true;
                        }
                    } else {
                        perimeter += 1;
                        sides += (map.get(cur+dir.clockwise()) != Some(&plant_type)
                               || map.get(cur+dir.clockwise()+dir) == Some(&plant_type)) as usize;
                    }
                }
            }
            //println!("Plant {plant_type} has area {area} and perimeter {perimeter}.");
            part1 += area*perimeter;
            part2 += area*sides;
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
    #[test] fn ex1() { assert_eq!(advent("example.txt").0, 140) }
    #[test] fn final1() { assert_eq!(advent("input.txt").0, 1374934) }
    #[test] fn ex2() { assert_eq!(advent("example.txt").1, 80) }
    #[test] fn final2() { assert_eq!(advent("input.txt").1, 841078) }

    #[bench] fn bench_advent(b: &mut Bencher) { b.iter(|| advent("input.txt")); }
}

