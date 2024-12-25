#![feature(test)]
extern crate test;
use std::{collections::VecDeque, env, fs::read_to_string};

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

type Num = u64;

pub fn advent(filename: &str) -> (Num, Num) {
    let input =  read_to_string(filename).expect("could not read input file");
    let (initial, gates) = input.split_once("\n\n").expect("wrong file format");

    let (mut part1, mut part2) = (0,0);

    let mut wires = FxHashMap::default();

    for wire in initial.lines() {
        let (name, val) = wire.split_once(": ").expect("wrong initial wire value");
        wires.insert(name, val.parse::<u8>().unwrap());
    }

    let mut todo = VecDeque::new();
    todo.extend(gates.lines());

    while let Some(gate) = todo.pop_front() {
        let mut elem = gate.split(" ");
        let in1 = elem.next().unwrap();
        let op = elem.next().unwrap();
        let in2 = elem.next().unwrap();
        elem.next(); // skip "->"
        let out = elem.next().unwrap();
        if !(wires.contains_key(in1) && wires.contains_key(in2)) {
            todo.push_back(gate);
            continue;
        }
        let in1val = wires.get(in1).unwrap();
        let in2val = wires.get(in2).unwrap();
        let outval = match op {
            "AND" => {in1val & in2val},
            "OR" => {in1val | in2val},
            "XOR" => {in1val ^ in2val},
            _ => unreachable!()
        };

        wires.insert(out, outval);

        if out.starts_with("z") {
            //println!("{out} = {outval}");
            let numeric = out[1..].parse::<Num>().expect("cannot parse output wire");
            part1 += (outval as Num) << numeric;

            // Part 2
            //if op != "XOR" {wrong.push(out);println!("Wrong if not highest Z out: {out}")}
        }

        // Part 2
        // if op == "XOR" && !out.starts_with(&['x', 'y', 'z']) && !in1.starts_with(&['x', 'y', 'z']) && !in2.starts_with(&['x', 'y', 'z']) {
        //     wrong.push(out);println!("Wrong XOR: {out}")
        // }

        // There was one remaining swap for part 2, which I found manually.
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
    #[test] fn ex1() { assert_eq!(advent("example.txt").0, 4) }
    #[test] fn ex12() { assert_eq!(advent("example2.txt").0, 2024) }
    #[test] fn final1() { assert_eq!(advent("input.txt").0, 1077) }
    #[test] fn ex2() { assert_eq!(advent("example.txt").1, 0) }
    #[test] fn final2() { assert_eq!(advent("input.txt").1, 0) }

    #[bench] fn bench_advent(b: &mut Bencher) { b.iter(|| advent("input.txt")); }
}

