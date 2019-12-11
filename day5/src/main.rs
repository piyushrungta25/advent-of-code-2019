use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use intcode::{get_computer, Signal};

fn get_input() -> Result<Vec<i64>, Box<dyn Error>> {
    let mut f = File::open("input")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    Ok(s.split(',').filter_map(|x| x.parse::<i64>().ok()).collect())
}

fn main() {
    let input: Vec<i64> = get_input().unwrap();

    let mut c = get_computer(&input, vec![1]);
    c.run_till_signal(Signal::Halt);
    let part1 = c.get_output().unwrap();
    println!("Part 1: {:?}", part1);

    let mut c = get_computer(&input, vec![5]);
    c.run_till_signal(Signal::Halt);
    let part2 = c.get_output().unwrap();
    println!("Part 2: {:?}", part2);
}
