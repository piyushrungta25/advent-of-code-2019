use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn get_input() -> Result<Vec<u64>, Box<dyn Error>> {
    let mut f = File::open("input")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    Ok(s.split(',').filter_map(|x| x.parse::<u64>().ok()).collect())
}

struct IntCodeComputer {
    memory: Vec<u64>,
    ip: usize,
}

impl IntCodeComputer {
    fn new() -> Self {
        IntCodeComputer {
            memory: Vec::new(),
            ip: 0,
        }
    }

    fn load_memory(&mut self, memory: Vec<u64>) -> &mut Self {
        self.memory = memory;
        self
    }

    fn set_state(&mut self, noun: u64, verb: u64) -> &mut Self {
        self.memory[1] = noun;
        self.memory[2] = verb;
        self
    }

    fn run(&mut self) -> Result<u64, ()> {
        'program_loop: loop {
            let opcode = self.memory[self.ip];

            let op1_loc = self.memory[self.ip + 1] as usize;
            let op2_loc = self.memory[self.ip + 2] as usize;
            let dest = self.memory[self.ip + 3] as usize;

            let op1 = self.memory[op1_loc];
            let op2 = self.memory[op2_loc];

            match opcode {
                1 => self.memory[dest] = op1 + op2,
                2 => self.memory[dest] = op1 * op2,
                99 => {
                    break 'program_loop;
                }
                _ => return Err(()),
            }
            self.ip += 4;
        }

        Ok(self.memory[0])
    }
}

fn main() {
    let input: Vec<u64> = get_input().unwrap();
    match IntCodeComputer::new()
        .load_memory(input.clone())
        .set_state(12, 2)
        .run()
    {
        Ok(i) => println!("Part 1: {:?}", i),
        _ => panic!("unexpected error"),
    }

    for noun in 0..99 {
        for verb in 0..99 {
            match IntCodeComputer::new()
                .load_memory(input.clone())
                .set_state(noun, verb)
                .run()
            {
                Ok(19690720) => println!("Part 2: {:?}", 100 * noun + verb),
                _ => {}
            }
        }
    }
}
