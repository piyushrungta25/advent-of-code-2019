use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn get_input() -> Result<Vec<i32>, Box<dyn Error>> {
    let mut f = File::open("input")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    Ok(s.split(',').filter_map(|x| x.parse::<i32>().ok()).collect())
}

enum Parameter {
    Position(i32),
    Immediate(i32),
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

struct IntCodeComputer {
    memory: Vec<i32>,
    input: Vec<i32>,
    output: Option<i32>,
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

    fn load_memory(&mut self, memory: Vec<i32>) -> &mut Self {
        self.memory = memory;
        self.ip = 0;
        self
    }

    fn set_input(&mut self, input: Vec<i32>) -> &mut Self {
        self.input = input;
        self
    }

    fn fetch_word(&mut self) -> i32 {
        let word = self.memory[self.ip];
        self.ip += 1;
        word
    }

    fn _fetch_parameter(&mut self, mode: i32) -> Parameter {
        let word = self.fetch_word();
        match mode {
            0 => Parameter::Position(word),
            1 => Parameter::Immediate(word),
            _ => panic!("unknown parameter mode"),
        }
    }

    fn fetch_param1(&mut self, opcode: i32) -> Parameter {
        self._fetch_parameter(opcode % 10)
    }

    fn fetch_param2(&mut self, mut opcode: i32) -> (Parameter, Parameter) {
        let p1 = self.fetch_param1(opcode);
        opcode /= 10;
        let p2 = self._fetch_parameter(opcode % 10);
        (p1, p2)
    }

    fn fetch_param3(&mut self, mut opcode: i32) -> (Parameter, Parameter, Parameter) {
        let (p1, p2) = self.fetch_param2(opcode);
        opcode /= 100;
        let p3 = self._fetch_parameter(opcode % 10);
        (p1, p2, p3)
    }

    fn unwrap_value(&self, param: Parameter) -> i32 {
        match param {
            Parameter::Immediate(val) => val,
            Parameter::Position(pos) => self.memory[pos as usize],
        }
    }

    fn store_val(&mut self, param: Parameter, val: i32) {
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

    fn run(&mut self) -> &mut Self {
        'program_loop: loop {
            let inst = self.fetch_instruction();
            match inst {
                Instruction::Add((param1, param2, param3)) => {
                    let op1 = self.unwrap_value(param1);
                    let op2 = self.unwrap_value(param2);
                    self.store_val(param3, op1 + op2);
                }
                Instruction::Mul((param1, param2, param3)) => {
                    let op1 = self.unwrap_value(param1);
                    let op2 = self.unwrap_value(param2);
                    self.store_val(param3, op1 * op2);
                }
                Instruction::Input(param) => {
                    let inp = self.input.remove(0);
                    self.store_val(param, inp);
                }
                Instruction::Output(param) => {
                    self.emit_output(param);
                }
                Instruction::JumpIfTrue((param1, param2)) => {
                    if self.unwrap_value(param1) != 0 {
                        self.jump(param2);
                    }
                }
                Instruction::JumpIfFalse((param1, param2)) => {
                    if self.unwrap_value(param1) == 0 {
                        self.jump(param2);
                    }
                }
                Instruction::LessThan((param1, param2, param3)) => {
                    let op1 = self.unwrap_value(param1);
                    let op2 = self.unwrap_value(param2);
                    self.store_val(param3, if op1 < op2 { 1 } else { 0 })
                }
                Instruction::Equals((param1, param2, param3)) => {
                    let op1 = self.unwrap_value(param1);
                    let op2 = self.unwrap_value(param2);
                    self.store_val(param3, if op1 == op2 { 1 } else { 0 })
                }
                Instruction::Halt => break 'program_loop,
            }
        }
        self
    }
}

fn main() {
    let input: Vec<i32> = get_input().unwrap();
    let mut computer = IntCodeComputer::new();

    let part1 = computer
        .load_memory(input.clone())
        .set_input(vec![1])
        .run()
        .output
        .unwrap();
    println!("Part 1: {:?}", part1);

    let part2 = computer
        .load_memory(input.clone())
        .set_input(vec![5])
        .run()
        .output
        .unwrap();
    println!("Part 2: {:?}", part2);
}
