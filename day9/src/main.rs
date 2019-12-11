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
    c.run_till_signal(Signal::ProducedOutput);
    println!("Part1: {:?}", c.get_output().unwrap());

    let mut c = get_computer(&input, vec![2]);
    c.run_till_signal(Signal::ProducedOutput);
    println!("Part2: {:?}", c.get_output().unwrap());
}
