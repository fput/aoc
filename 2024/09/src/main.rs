#![feature(test)]
extern crate test;
use std::{env,fs::read_to_string};

const FREE: u32 = u32::MAX;
pub fn advent(filename: &str) -> (usize, usize) {
    let input = read_to_string(filename).expect("could not read input file");
    let diskmap: Vec<u32> =  input.trim().chars().filter_map(|c| c.to_digit(10)).collect();
    let (mut part1, mut part2) = (0,0);
    let mut blocks1: Vec<u32> = Vec::new(); // Input format for part 1
    let mut blocks2: Vec<(u32,u32)> = Vec::new(); // Input format for part 2
    let mut file_id = 0;

    for (i, &len) in diskmap.iter().enumerate() {
        if i % 2 == 0 { // is a file
            blocks1.extend(vec![file_id; len as usize]);
            blocks2.push((file_id, len));
            file_id += 1;
        } else { // is free space
            blocks1.extend(vec![FREE; len as usize]);
            blocks2.push((FREE, len));
        }
    }

    // Part 1
    let mut last_idx_front = 0;
    // Iterate from the end of the vector
    for idx_back in (0..blocks1.len()).rev() {
        if last_idx_front >= idx_back { break; }
        if blocks1[idx_back] == FREE { continue; }

        for idx_front in last_idx_front..idx_back {
            // Find the next free space from the beginning
            if blocks1[idx_front] == FREE {
                //println!("Swapping {} (index {i}) to index {j}", blocks1[j]);
                blocks1.swap(idx_front, idx_back);
                last_idx_front = idx_front;
                break;
            }
        }
    }

    // Checksum part 1
    for (idx, &file_id) in blocks1.iter().enumerate() {
        if file_id == FREE { break; }
        part1 += idx*(file_id as usize);
    }

    // Part 2
    // Iterate from the end of the vector
    for idx_back in (0..blocks2.len()).rev() {
        let (back_id, back_len) = blocks2[idx_back];
        if back_id == FREE { continue; }

        // Find the next free space from the beginning
        for idx_front in 0..idx_back {
            let (front_id, front_len) = blocks2[idx_front];

            if front_id == FREE && front_len >= back_len {
                //println!("Shifting block {back_id} to beginning");
                blocks2[idx_back].0 = FREE;

                if front_len == back_len {
                    blocks2[idx_front].0 = back_id;
                } else {
                    blocks2[idx_front].1 -= back_len;
                    blocks2.insert(idx_front, (back_id, back_len));
                }

                break;
            }

        }
    }

    // Checksum part 2
    let mut idx: usize = 0; 
    for (id, size) in blocks2 {
        if id != FREE {
            for _ in 0..size {
                part2 += idx*id as usize;
                idx += 1;
            }
        } else {
            idx += size as usize;
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
    #[test] fn ex1() { assert_eq!(advent("example.txt").0, 1928) }
    #[test] fn final1() { assert_eq!(advent("input.txt").0, 6154342787400) }
    #[test] fn ex2() { assert_eq!(advent("example.txt").1, 2858) }
    #[test] fn final2() { assert_eq!(advent("input.txt").1, 6183632723350) }

    #[bench] fn bench_advent(b: &mut Bencher) { b.iter(|| advent("input.txt")); }
}

