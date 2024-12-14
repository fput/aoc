#![feature(test)]
#![feature(iter_collect_into)]
extern crate test;
use std::{env, fs::read_to_string};
use util::*;

type Num = i32;

pub fn advent(filename: &str, width: Num, height: Num) -> (Num, Num) {
    let input =  read_to_string(filename).expect("could not read input file");
    let (thresh_x, thresh_y) = (width / 2, height / 2);
    let (mut northwest, mut northeast, mut southwest, mut southeast) = (0,0,0,0);
    let mut robots = Vec::new();
    let mut buf = Vec::new();

    for line in input.lines() {
        buf.clear();
        extract_numbers_signed::<Num>(line).collect_into(&mut buf);
        let (px, py, vx, vy) = (buf[0], buf[1], buf[2], buf[3]);
        robots.push((px, py, vx, vy));

        // Calculate future positions
        let px = (px + 100*vx).rem_euclid(width);
        let py = (py + 100*vy).rem_euclid(height);

        if px < thresh_x {
            if py < thresh_y { northwest += 1; }
            else if py > thresh_y { northeast += 1; }
        } else if px > thresh_x {
            if py < thresh_y { southwest += 1; }
            else if py > thresh_y { southeast += 1; }
        }
    }
    let part1 = northwest * northeast * southwest * southeast;

    // Part 2
    // Pattern mid: 43s, 146s, 249s (+103s)
    // Pattern vertical: 68s, 169s, 270s (+101s)
    let mut part2 = 0;
    for i in 101..10000 {
        let mut map = vec![vec![0u32; width as usize]; height as usize];
        for &(px, py, vx, vy) in &robots {
            let px = (px + i * vx).rem_euclid(width);
            let py = (py + i * vy).rem_euclid(height);
            map[py as usize][px as usize] += 1;
        }

        if map.iter().any(|row| row.iter()
            .fold((0,0), |(consec, max), &val|
                if val == 1 { (consec + 1, max.max(consec+1)) }
                else { (0, max) }).1 >= 30)
            { part2 = i; break; }
        
        // Print map
        // println!("Map after {i}s");
        // for line in map {
        //     for tile in line {
        //         if tile == 0 {
        //             print!(".");
        //         } else {
        //             print!("{tile}");
        //         }
        //     }
        //     println!();
        // }
        // println!();
        // let _ = std::io::stdin().read_line(&mut tmp);
    }

    (part1, part2)
}

fn main() {
    let filename = env::args().nth(1).expect("No argument found");
    let (part1, part2) = advent(&filename, 101, 103);
    print!("{}\n{}\n", part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    #[test] fn example_input() { dbg!(advent("example.txt", 11, 7)); }
    #[test] fn final_input() { dbg!(advent("input.txt", 101, 103)); }
    #[test] fn ex1() { assert_eq!(advent("example.txt", 11, 7).0, 12) }
    #[test] fn final1() { assert_eq!(advent("input.txt", 101, 103).0, 230900224) }
    #[test] fn final2() { assert_eq!(advent("input.txt", 101, 103).1, 6532) }

    #[bench] fn bench_advent(b: &mut Bencher) { b.iter(|| advent("input.txt", 101, 103)); }
}

