use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

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

enum Parameter {
    Position(i64),
    Immediate(i64),
}

enum Instruction {
    Add((Parameter, Parameter, Parameter)),
    Mul((Parameter, Parameter, Parameter)),
    Input(Parameter),
    Output(Parameter),
    JumpIfTrue((Parameter, Parameter)),
    JumpIfFalse((Parameter, Parameter)),
    LessThan((Parameter, Parameter, Parameter)),
    Equals((Parameter, Parameter, Parameter)),
    Halt,
}

#[derive(PartialEq)]
enum Signal {
    NeedsInput,
    ProducedOutput,
    Halt,
    None,
}

struct IntCodeComputer {
    memory: Vec<i64>,
    input: Vec<i64>,
    output: Option<i64>,
    ip: usize,
}

impl IntCodeComputer {
    fn new() -> Self {
        IntCodeComputer {
            memory: Vec::new(),
            input: Vec::new(),
            output: None,
            ip: 0,
        }
    }

    fn load_memory(&mut self, memory: Vec<i64>) -> &mut Self {
        self.memory = memory;
        self.ip = 0;
        self
    }

    fn set_input(&mut self, input: Vec<i64>) -> &mut Self {
        self.input = input;
        self
    }

    fn fetch_word(&mut self) -> i64 {
        let word = self.memory[self.ip];
        self.ip += 1;
        word
    }

    fn _fetch_parameter(&mut self, mode: i64) -> Parameter {
        let word = self.fetch_word();
        match mode {
            0 => Parameter::Position(word),
            1 => Parameter::Immediate(word),
            _ => panic!("unknown parameter mode"),
        }
    }

    fn fetch_param1(&mut self, opcode: i64) -> Parameter {
        self._fetch_parameter(opcode % 10)
    }

    fn fetch_param2(&mut self, mut opcode: i64) -> (Parameter, Parameter) {
        let p1 = self.fetch_param1(opcode);
        opcode /= 10;
        let p2 = self._fetch_parameter(opcode % 10);
        (p1, p2)
    }

    fn fetch_param3(&mut self, mut opcode: i64) -> (Parameter, Parameter, Parameter) {
        let (p1, p2) = self.fetch_param2(opcode);
        opcode /= 100;
        let p3 = self._fetch_parameter(opcode % 10);
        (p1, p2, p3)
    }

    fn unwrap_value(&self, param: Parameter) -> i64 {
        match param {
            Parameter::Immediate(val) => val,
            Parameter::Position(pos) => self.memory[pos as usize],
        }
    }

    fn store_val(&mut self, param: Parameter, val: i64) {
        match param {
            Parameter::Position(out) => {
                self.memory[out as usize] = val;
            }
            _ => panic!("can not store to parameter in immediate mode"),
        }
    }

    fn emit_output(&mut self, param: Parameter) {
        self.output = Some(self.unwrap_value(param));
    }

    fn jump(&mut self, param: Parameter) {
        self.ip = self.unwrap_value(param) as usize;
    }

    fn feed_input(&mut self, inp: i64) {
        self.input.push(inp);
    }

    fn fetch_instruction(&mut self) -> Instruction {
        let mut opcode = self.fetch_word();
        let inst = opcode % 100;
        opcode /= 100;

        match inst {
            1 => Instruction::Add(self.fetch_param3(opcode)),
            2 => Instruction::Mul(self.fetch_param3(opcode)),
            3 => Instruction::Input(self.fetch_param1(opcode)),
            4 => Instruction::Output(self.fetch_param1(opcode)),
            5 => Instruction::JumpIfTrue(self.fetch_param2(opcode)),
            6 => Instruction::JumpIfFalse(self.fetch_param2(opcode)),
            7 => Instruction::LessThan(self.fetch_param3(opcode)),
            8 => Instruction::Equals(self.fetch_param3(opcode)),
            99 => Instruction::Halt,
            _ => panic!("unknown instruction"),
        }
    }

    fn get_output(&mut self) -> i64 {
        let ret = self.output.unwrap();
        self.output = None;
        ret
    }

    fn tick(&mut self) -> Signal {
        let inst = self.fetch_instruction();
        match inst {
            Instruction::Add((param1, param2, param3)) => {
                let op1 = self.unwrap_value(param1);
                let op2 = self.unwrap_value(param2);
                self.store_val(param3, op1 + op2);
                return Signal::None;
            }
            Instruction::Mul((param1, param2, param3)) => {
                let op1 = self.unwrap_value(param1);
                let op2 = self.unwrap_value(param2);
                self.store_val(param3, op1 * op2);
                return Signal::None;
            }
            Instruction::Input(param) => {
                if self.input.is_empty() {
                    return Signal::NeedsInput;
                }
                let inp = self.input.remove(0);
                self.store_val(param, inp);
                return Signal::None;
            }
            Instruction::Output(param) => {
                self.emit_output(param);
                return Signal::ProducedOutput;
            }
            Instruction::JumpIfTrue((param1, param2)) => {
                if self.unwrap_value(param1) != 0 {
                    self.jump(param2);
                }
                return Signal::None;
            }
            Instruction::JumpIfFalse((param1, param2)) => {
                if self.unwrap_value(param1) == 0 {
                    self.jump(param2);
                }
                return Signal::None;
            }
            Instruction::LessThan((param1, param2, param3)) => {
                let op1 = self.unwrap_value(param1);
                let op2 = self.unwrap_value(param2);
                self.store_val(param3, if op1 < op2 { 1 } else { 0 });
                return Signal::None;
            }
            Instruction::Equals((param1, param2, param3)) => {
                let op1 = self.unwrap_value(param1);
                let op2 = self.unwrap_value(param2);
                self.store_val(param3, if op1 == op2 { 1 } else { 0 });
                return Signal::None;
            }
            Instruction::Halt => return Signal::Halt,
        }
    }

    fn run_till_signal(&mut self, signal: Signal) {
        while self.tick() != signal {}
    }

    fn run(&mut self) -> Signal {
        let mut s = self.tick();
        while s == Signal::None {
            s = self.tick();
        }

        s
    }
}

fn get_computer(mem: &Vec<i64>, input: Vec<i64>) -> IntCodeComputer {
    let mut computer = IntCodeComputer::new();
    computer.load_memory(mem.clone()).set_input(input);
    computer
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
                sig = Some(amplifiers[i].get_output());
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
        out = amplifiers[i].get_output();
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
