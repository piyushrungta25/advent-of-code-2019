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
    let mut computer = get_computer(&input, vec![]);
    computer.store_value_at_pos(1, 12);
    computer.store_value_at_pos(2, 2);
    computer.run_till_signal(Signal::Halt);
    println!("Part 1: {:?}", computer.get_value_at_pos(0));


    // part 2
    for noun in 0..100 {
        for verb in 0..100 {
            let mut computer = get_computer(&input, vec![]);
            computer.store_value_at_pos(1, noun);
            computer.store_value_at_pos(2, verb);
            computer.run_till_signal(Signal::Halt);

            match computer.get_value_at_pos(0) {
                19690720 => {
                    println!("Part 2: {:?}", 100 * noun + verb);
                    break;
                }
                _ => {}
            }
        }
    }
}
