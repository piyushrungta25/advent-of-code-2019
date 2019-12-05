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
    Immediate(i32)
}

enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Mul(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equals(Parameter, Parameter, Parameter),
    Halt
}

struct IntCodeComputer {
    memory: Vec<i32>,
    input: Vec<i32>,
    output: Vec<i32>,
    ip: usize,
}

impl IntCodeComputer {
    fn new() -> Self {
        IntCodeComputer {
            memory: Vec::new(),
            input: Vec::new(),
            output: Vec::new(),
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

    fn set_state(&mut self, noun: i32, verb: i32) -> &mut Self {
        self.memory[1] = noun;
        self.memory[2] = verb;
        self
    }

    fn fetch_parameter(&mut self, mode: i32) -> Parameter {
        let param = match mode {
            0 => Parameter::Position(self.memory[self.ip]),
            1 => Parameter::Immediate(self.memory[self.ip]),
            _ => panic!("unknown parameter mode")
        };

        self.ip+=1;
        param
    }

    fn unwrap_value(&self, param: Parameter) -> i32 {
        match param {
            Parameter::Immediate(val) => val,
            Parameter::Position(pos) => self.memory[pos as usize]
        }
    }

    fn fetch_instruction(&mut self) -> Instruction {
        let mut opcode: i32 = self.memory[self.ip];
        self.ip+=1;
        let inst = opcode % 100;
        opcode /= 100;

        match inst {
            1|2 => {
                let param1 = self.fetch_parameter(opcode % 10);
                opcode /= 10;
                let param2 = self.fetch_parameter(opcode % 10);
                opcode /= 10;
                let param3 = self.fetch_parameter(opcode % 10);
                match inst {
                    1 => Instruction::Add(param1, param2, param3),
                    2 => Instruction::Mul(param1, param2, param3),
                    _ => panic!("bad")
                }

            },
            3|4 => {
                let param = self.fetch_parameter(opcode % 10);
                match inst {
                    3 => Instruction::Input(param),
                    4 => Instruction::Output(param),
                    _ => panic!("bad")
                }
            },
            5|6 => {
                let param1 = self.fetch_parameter(opcode % 10);
                opcode /= 10;
                let param2 = self.fetch_parameter(opcode % 10);

                match inst {
                    5 => Instruction::JumpIfTrue(param1, param2),
                    6 => Instruction::JumpIfFalse(param1, param2),
                    _ => panic!("bad")
                }


            },
            7|8 => {
                let param1 = self.fetch_parameter(opcode % 10);
                opcode /= 10;
                let param2 = self.fetch_parameter(opcode % 10);
                opcode /= 10;
                let param3 = self.fetch_parameter(opcode % 10);

                match inst {
                    7 => Instruction::LessThan(param1, param2, param3),
                    8 => Instruction::Equals(param1, param2, param3),
                    _ => panic!("bad")
                }

            },
            99 => {Instruction::Halt}
            _ => {panic!("unknown instruction")}
        }
    }

    fn run(&mut self) {
        'program_loop: loop {
            let inst = self.fetch_instruction();
            match inst {
                Instruction::Add(param1, param2, param3) => {
                    let op1 = self.unwrap_value(param1);
                    let op2 = self.unwrap_value(param2);

                    match param3 {
                        Parameter::Position(out) => {
                            self.memory[out as usize] = op1 + op2;
                        },
                        _ => panic!("bad bad bad")
                    }
                },
                Instruction::Mul(param1, param2, param3) => {
                    let op1 = self.unwrap_value(param1);
                    let op2 = self.unwrap_value(param2);

                    match param3 {
                        Parameter::Position(out) => {
                            self.memory[out as usize] = op1 * op2;
                        },
                        _ => panic!("bad bad bad")
                    }
                },
                Instruction::Input(param) => {
                    match param {
                        Parameter::Position(pos) => {
                            self.memory[pos as usize] = self.input.remove(0);
                        },
                        _ => panic!("bad bad bad")
                    }
                },
                Instruction::Output(param) => {
                    match param {
                        Parameter::Position(pos) => {
                            self.output.push(self.memory[pos as usize]);
                        },
                        Parameter::Immediate(val) => {
                            self.output.push(val);

                        }
                    }
                },
                Instruction::JumpIfTrue(param1, param2) => {
                    match self.unwrap_value(param1) != 0 {
                        true => self.ip = self.unwrap_value(param2) as usize,
                        _ => {}
                    }
                },
                Instruction::JumpIfFalse(param1, param2) => {
                    match self.unwrap_value(param1) == 0 {
                        true => self.ip = self.unwrap_value(param2) as usize,
                        _ => {}
                    }
                },
                Instruction::LessThan(param1, param2, param3) => {
                    let op1 = self.unwrap_value(param1);
                    let op2 = self.unwrap_value(param2);

                    match param3 {
                        Parameter::Position(pos) => {
                            self.memory[pos as usize] = match op1 < op2 {
                                true => 1,
                                _ => 0
                            }
                        },
                        _ => panic!("bad bad bad")
                    }
                },
                Instruction::Equals(param1, param2, param3) => {
                    let op1 = self.unwrap_value(param1);
                    let op2 = self.unwrap_value(param2);

                    match param3 {
                        Parameter::Position(pos) => {
                            self.memory[pos as usize] = match op1 == op2 {
                                true => 1,
                                _ => 0
                            }
                        },
                        _ => panic!("bad bad bad")
                    }
                },
                Instruction::Halt => break 'program_loop,
            }
        }
    }
}

fn main() {
    let input: Vec<i32> = get_input().unwrap();
    let mut computer = IntCodeComputer::new();

    // part 1
    computer.load_memory(input.clone()).set_input(vec![1]).run();

    let part1 = computer.output.last().unwrap();
    assert_eq!(*part1, 10987514);
    println!("Part 1: {:?}", part1);

    computer.load_memory(input.clone()).set_input(vec![5]).run();
    let part2 = computer.output.last().unwrap();
    assert_eq!(*part2, 14195011);
    println!("Part 2: {:?}", part2);

}
