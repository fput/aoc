#![feature(test)]
extern crate test;
use std::{env, fs::read_to_string};
use itertools::Itertools;
use util::*;

type Num = i64;

pub fn advent(filename: &str) -> (String, Num) {
    let input =  read_to_string(filename).expect("could not read input file");
    let lines = input.lines().collect::<Vec<_>>();

    let a = extract_numbers::<Num>(lines[0]).next().unwrap();
    let b = extract_numbers::<Num>(lines[1]).next().unwrap();
    let c = extract_numbers::<Num>(lines[2]).next().unwrap();
    let insts: Vec<_> = extract_numbers::<Num>(lines[4]).collect();

    let mut out: Vec<Num> = Vec::new();
    run_program(&insts, a, b, c, &mut out);
    let part1 = out.iter().join(",");

    let part2 = find_a_quine(0, 0, b, c, &insts, &mut out).unwrap_or_default();

    (part1, part2)
}

/// Brute force `a` with three bits at a time, from LSB to MSB.
/// The lowest three bits must be 000, so start with that. Then,
/// find all higher three bits that result in the next correct output
/// value from left to right.
fn find_a_quine(i: usize, a: Num, b: Num, c: Num, insts: &[Num], out: &mut Vec<Num>) -> Option<Num> {
    run_program(insts, a, b, c, out);
    if out == insts { return Some(a); } // We found the quine
    if i >= insts.len() {return None; }
    if i==0 || out[0] == insts[insts.len()-i] {
        for n in 0..8 {
            if let Some(found) = find_a_quine(i + 1, (a << 3) + n, b, c, insts, out) {
                return Some(found);
            }
        }
    }
    None
}

fn run_program(insts: &[Num], a: Num, b: Num, c: Num, output: &mut Vec<Num>) {
    output.clear();
    let mut ip: usize = 0;
    let mut a = a;
    let mut b = b;
    let mut c = c;

    while ip+1<insts.len() {
        let (opcode, operand) = (insts[ip], insts[ip+1]);
        let combo = || match operand {
            0..=3 => operand,
            4 => a,
            5 => b,
            6 => c,
            _ => unreachable!()
        };

        //println!("IP={ip}: opcode {opcode}, operand {operand}");
        match opcode {
            0 => a = a >> combo(),                              // ADV instruction
            1 => b ^= operand,                                  // BXL instruction
            2 => b = combo() % 8,                               // BST instruction
            3 => if a!=0 {ip = operand as usize; continue;},    // JNZ instruction
            4 => b ^= c,                                        // BXC instruction
            5 => output.push(combo() % 8),                      // OUT instruction
            6 => b = a >> combo(),                              // BDV instruction
            7 => c = a >> combo(),                              // CDV instruction
            _ => unreachable!()
        }
        ip += 2;
    }
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
    #[test] fn ex1() { assert_eq!(advent("example.txt").0, "4,6,3,5,6,3,5,2,1,0") }
    #[test] fn final1() { assert_eq!(advent("input.txt").0, "1,5,0,1,7,4,1,0,3") }
    #[test] fn ex2() { assert_eq!(advent("example2.txt").1, 117440) }
    #[test] fn final2() { assert_eq!(advent("input.txt").1, 47910079998866) }

    #[bench] fn bench_advent(b: &mut Bencher) { b.iter(|| advent("input.txt")); }
}

