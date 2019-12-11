use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use intcode::{get_computer, Signal};

// ===================================================
// modified permutation code from Rosetta Code
// https://rosettacode.org/wiki/Permutations#Iterative
// ===================================================
pub fn permutations(start: usize, end: usize) -> Permutations {
    Permutations {
        idxs: (start..=end).collect(),
        swaps: vec![0; end - start + 1],
        i: 0,
    }
}

pub struct Permutations {
    idxs: Vec<usize>,
    swaps: Vec<usize>,
    i: usize,
}

impl Iterator for Permutations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i > 0 {
            loop {
                if self.i >= self.swaps.len() {
                    return None;
                }
                if self.swaps[self.i] < self.i {
                    break;
                }
                self.swaps[self.i] = 0;
                self.i += 1;
            }
            self.idxs.swap(self.i, (self.i & 1) * self.swaps[self.i]);
            self.swaps[self.i] += 1;
        }
        self.i = 1;
        Some(self.idxs.clone())
    }
}

fn get_input() -> Result<Vec<i64>, Box<dyn Error>> {
    let mut f = File::open("input")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    Ok(s.split(',').filter_map(|x| x.parse::<i64>().ok()).collect())
}

fn get_signal_with_feedback(input: &Vec<i64>, phase: Vec<usize>) -> i64 {
    let mut amplifiers = [
        get_computer(&input, vec![phase[0] as i64, 0]),
        get_computer(&input, vec![phase[1] as i64]),
        get_computer(&input, vec![phase[2] as i64]),
        get_computer(&input, vec![phase[3] as i64]),
        get_computer(&input, vec![phase[4] as i64]),
    ];

    let mut sig: Option<i64> = None;
    for i in (0..5).cycle() {
        if let Some(s) = sig {
            amplifiers[i].feed_input(s);
        }

        match amplifiers[i].run() {
            Signal::ProducedOutput => {
                sig = Some(amplifiers[i].get_output().unwrap());
            }
            Signal::Halt => {
                if i == 4 {
                    break;
                }
            }
            _ => {}
        }
    }

    sig.unwrap()
}

fn get_signal(input: &Vec<i64>, phase: Vec<usize>) -> i64 {
    let mut amplifiers = [
        get_computer(&input, vec![phase[0] as i64]),
        get_computer(&input, vec![phase[1] as i64]),
        get_computer(&input, vec![phase[2] as i64]),
        get_computer(&input, vec![phase[3] as i64]),
        get_computer(&input, vec![phase[4] as i64]),
    ];

    let mut out = 0;
    for i in 0..5 {
        amplifiers[i].feed_input(out);
        amplifiers[i].run_till_signal(Signal::ProducedOutput);
        out = amplifiers[i].get_output().unwrap();
    }

    out
}

fn part2(input: &Vec<i64>) -> i64 {
    let mut mx = std::i64::MIN;
    for perm in permutations(5, 9) {
        mx = ::std::cmp::max(mx, get_signal_with_feedback(&input, perm));
    }
    mx
}

fn part1(input: &Vec<i64>) -> i64 {
    let mut mx = std::i64::MIN;
    for perm in permutations(0, 4) {
        mx = ::std::cmp::max(mx, get_signal(&input, perm));
    }
    mx
}

fn main() {
    let input: Vec<i64> = get_input().unwrap();

    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
