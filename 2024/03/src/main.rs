#![feature(test)]
extern crate test;
use std::{env,fs::read_to_string};

fn advent(filename: &str) -> (u32, u32) {
    let mem = read_to_string(filename)
        .expect("Something went wrong reading the input file");
    let part1 = muls_and_sum(&mem);
    let part2 = between_sums(&mem, "do()", "don't()", muls_and_sum);
    (part1, part2)
}

/// Multiplies all occurences of "mul(a,b)" (= a*b) in [text] and sums them.
///
/// Non-digits within the "mul(...)"" get correctly ignored.
/// But "mul(a)" or "mul(a,b,c)" are not discarded.
///
/// Note: If the beginning of [text] up until the first "mul(" is a number,
/// or a comma separated list of numbers, these get multiplied and added to the result.
/// Fortunately, this does not happen in my AoC input.
fn muls_and_sum(text: &str) -> u32 {
    between_sums(text, "mul(", ")",
        |s| s.split(",")
             // ignore non-digits in mul() instructions
             .map(|num| num.parse::<u32>().unwrap_or(0))
             .product::<u32>())
}

/// Maps a function [f] on all substrings in [text] between sequential
/// occurrences of [from] and [to]. Afterwards it sums the results.
/// It also applies [f] to the beginning of the [text] up until the
/// first occurence of [from].
/// 
/// Example:
///   Inputs: text = "(abc)def(ghi)j(klm)", from = "(", to = ")"
///   Applies [f] to "", "abc", "ghi", and "klm", and sums the results.
fn between_sums(text: &str, from: &str, to: &str, f: impl Fn(&str) -> u32) -> u32 {
    text.split(from).filter_map(|s| s.split(to).next()).map(f).sum()
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
    #[test] fn ex1() { assert_eq!(advent("example.txt").0, 161) }
    #[test] fn final1() { assert_eq!(advent("input.txt").0, 160672468) }
    #[test] fn ex2() { assert_eq!(advent("example2.txt").1, 48) }
    #[test] fn final2() { assert_eq!(advent("input.txt").1, 84893551) }

    #[bench] fn bench_advent(b: &mut Bencher) { b.iter(|| advent("input.txt")); }

    // better implementation, but longer
    fn muls_and_sum2(text: &str) -> u32 {
        between_sums(text, "mul(", ")",
            |s| {
                let (a, b) = s.split_once(',').unwrap_or((s, ""));
                // ignore non-digits in mul() instructions
                a.parse::<u32>().unwrap_or(0) * b.parse::<u32>().unwrap_or(0)   
            })
    }

    #[test] fn muls_and_sum_test() {
        assert_eq!(muls_and_sum("1abcmul(2,3)"), 6);
        assert_eq!(muls_and_sum("1abcmul(2,3)defmul(5,5),123amul(2,2)"), 35);
        assert_eq!(muls_and_sum2("1abcmul(2,3,4)"), 0);
        assert_eq!(muls_and_sum2("1abcmul(2)"), 0);
        assert_eq!(muls_and_sum2("1abcmul(2)abcmul(3,5),mul(23,mul(4,5))"), 35);
    }
}

