use std::collections::VecDeque;

const ADD: i32 = 1;
const MUL: i32 = 2;
const INPUT: i32 = 3;
const OUTPUT: i32 = 4;
const JUMP_TRUE: i32 = 5;
const JUMP_FALSE: i32 = 6;
const LESS_THAN: i32 = 7;
const EQUAL: i32 = 8;
const HALT: i32 = 99;

const POSITION: i32 = 0;

pub struct Computer {
    pub ram: Vec<i32>,
    pub outputs: VecDeque<i32>,
    pc: usize,
}

struct Instruction {
    opcode: i32,
    modes: [i32; 3],
}

impl Computer {
    pub fn new(ram: Vec<i32>) -> Computer {
        Computer {
            ram: ram,
            outputs: VecDeque::new(),
            pc: 0,
        }
    }

    pub fn run(&mut self) {
        let mut empty = VecDeque::new();
        self.run_with(&mut empty);
    }

    pub fn run_with(&mut self, inputs: &mut VecDeque<i32>) {
        loop {
            let instruction = Computer::decode_instruction(self.ram[self.pc]);
            match instruction.opcode {
                ADD => self.run_binary_instruction(&instruction, |x, y| x + y),
                MUL => self.run_binary_instruction(&instruction, |x, y| x * y),
                INPUT => {
                    let x = inputs.pop_front().unwrap();
                    let dst = self.ram[self.pc + 1] as usize;
                    assert_eq!(instruction.modes[0], POSITION);
                    self.ram[dst] = x;
                    self.pc += 2;
                }
                OUTPUT => {
                    let x = self.get_parameter(self.pc + 1, instruction.modes[0]);
                    self.outputs.push_back(x);
                    self.pc += 2;
                }
                JUMP_TRUE => self.run_jump_instruction(&instruction, |x| x != 0),
                JUMP_FALSE => self.run_jump_instruction(&instruction, |x| x == 0),
                LESS_THAN => self.run_binary_instruction(&instruction, |x, y| (x < y) as i32),
                EQUAL => self.run_binary_instruction(&instruction, |x, y| (x == y) as i32),
                HALT => break,
                _ => panic!("Unsupported opcode {}", self.ram[self.pc]),
            }
        }
    }

    fn run_binary_instruction(&mut self, instruction: &Instruction, f: fn(i32, i32) -> i32) {
        assert_eq!(instruction.modes[2], POSITION);
        let in1 = self.get_parameter(self.pc + 1, instruction.modes[0]);
        let in2 = self.get_parameter(self.pc + 2, instruction.modes[1]);
        let dst = self.ram[self.pc + 3] as usize;
        self.ram[dst] = f(in1, in2);
        self.pc += 4;
    }

    fn run_jump_instruction(&mut self, instruction: &Instruction, pred: fn(i32) -> bool) {
        let x = self.get_parameter(self.pc + 1, instruction.modes[0]);
        let addr = self.get_parameter(self.pc + 2, instruction.modes[1]) as usize;
        if pred(x) {
            self.pc = addr;
        } else {
            self.pc += 3;
        }
    }

    fn decode_instruction(x: i32) -> Instruction {
        Instruction {
            opcode: x % 100,
            modes: [(x / 100) % 10, (x / 1000) % 10, (x / 10000) % 10],
        }
    }

    fn get_parameter(&self, addr: usize, mode: i32) -> i32 {
        let mut x = self.ram[addr];
        if mode == POSITION {
            x = self.ram[x as usize];
        }
        x
    }
}
