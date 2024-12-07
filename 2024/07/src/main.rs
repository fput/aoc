// Iterative solution because I assumed that the recursive approach
// would be slower. But the recursive approach allows a depth first
// approach, which allows early terminating the search for the
// correct operator combination.
#![feature(test)]
extern crate test;
use std::{env,fs::read_to_string};
use util::*;

type Num = u64;
const MAX_NUMBERS_PER_LINE: u32 = 14;

pub fn advent(filename: &str) -> (u64, u64) {
    let input = read_to_string(filename).expect("could not read input file");
    let (mut part1, mut part2) = (0,0);

    // Buffer for tokens of each line (reused for each line)
    let mut tokens = Vec::with_capacity(MAX_NUMBERS_PER_LINE as usize);
    // Buffer that collects all operator combinations (reused for each line)
    let mut buf = vec![0 as Num; 3usize.pow(MAX_NUMBERS_PER_LINE-2)]; 

    for line in input.lines() {
        tokens.clear();
        tokens.extend(extract_numbers::<Num>(&line));
        let test = tokens[0];

        // First operand
        buf[0] = tokens[1];

        let mut prev_size = 1;
        for &op2 in &tokens[2..] {
            // Expand backwards to avoid overwriting unread values
            for idx in (0..prev_size).rev() {
                let op1 = buf[idx];
                let base = idx * 3;
                buf[base]     = op1 + op2;
                buf[base + 1] = op1 * op2;
                buf[base + 2] = concat(op1, op2);
            }
            prev_size *= 3;
        }

        // Now all results are in buf[0..prev_size].
        // Check if one operator combination results in the test value
        let mut part1_found = false;
        let mut part2_found = false;
        for (index, &val) in buf[..prev_size].iter().enumerate() {
            if val == test {
                if !part2_found {
                    part2 += test;
                    part2_found = true;
                }

                // part1 includes only those without any concat
                if !part1_found && !involved_concat(index)  {
                    part1 += test;
                    part1_found = true;
                }

                if part1_found && part2_found {
                    break;
                }
            }
        }
    }

    (part1, part2)
}

// Concatenates the numbers `n1` and `n2`.
// Example: concat(12, 345) = 12345`
const fn concat(n1: Num, n2: Num) -> Num {
    match n2 {
        0..=9 => n1 * 10 + n2,
        10..=99 => n1 * 100 + n2,
        100..=999 => n1 * 1000 + n2,
        _ => panic!("concat not implemented for numbers >= 1000"),
    }
}

// Find out based on the index if this result involved a concatenation.
#[inline]
fn involved_concat(index: usize) -> bool {
    if index == 0 {
        false
    } else if index % 3 == 2 {
        true
    } else {
        involved_concat(index / 3)
    }
}

fn main() {
    let filename = env::args().nth(1).expect("No argument found");
    let (part1, part2) = advent(&filename);
    //for _ in 0..1000 {
    //     advent(&filename);
    //}
    print!("{}\n{}\n", part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    #[test] fn example_input() { dbg!(advent("example.txt")); }
    #[test] fn final_input() { dbg!(advent("input.txt")); }
    #[test] fn ex1() { assert_eq!(advent("example.txt").0, 3749) }
    #[test] fn final1() { assert_eq!(advent("input.txt").0, 850435817339) }
    #[test] fn ex2() { assert_eq!(advent("example.txt").1, 11387) }
    #[test] fn final2() { assert_eq!(advent("input.txt").1, 104824810233437) }

    #[bench] fn bench_advent(b: &mut Bencher) { b.iter(|| advent("input.txt")); }
}

