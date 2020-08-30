use std::collections::VecDeque;

const ADD: i64 = 1;
const MUL: i64 = 2;
const INPUT: i64 = 3;
const OUTPUT: i64 = 4;
const JUMP_TRUE: i64 = 5;
const JUMP_FALSE: i64 = 6;
const LESS_THAN: i64 = 7;
const EQUAL: i64 = 8;
const ADJUST_RELATIVE_BASE: i64 = 9;
const HALT: i64 = 99;

const POSITION: i64 = 0;
const IMMEDIATE: i64 = 1;
const RELATIVE: i64 = 2;

pub struct Computer {
    pub ram: Vec<i64>,
    pub outputs: VecDeque<i64>,
    pub state: State,
    pc: usize,
    rb: i64,
}

struct Instruction {
    opcode: i64,
    modes: [i64; 3],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    Running,
    WaitingForInput,
    Halted,
}

impl Computer {
    #[inline]
    pub fn new(ram: Vec<i64>) -> Computer {
        Computer {
            ram,
            outputs: VecDeque::new(),
            state: State::Running,
            pc: 0,
            rb: 0,
        }
    }

    #[inline]
    pub fn run(&mut self) -> State {
        let mut empty = VecDeque::new();
        self.run_with(&mut empty)
    }

    pub fn run_with(&mut self, inputs: &mut VecDeque<i64>) -> State {
        assert_ne!(self.state, State::Halted);
        self.state = loop {
            let instruction = Computer::decode_instruction(self.ram[self.pc]);
            match instruction.opcode {
                ADD => self.run_binary_instruction(&instruction, |x, y| x + y),
                MUL => self.run_binary_instruction(&instruction, |x, y| x * y),
                INPUT => match inputs.pop_front() {
                    Some(x) => {
                        let dst = self.get_output_addr(self.pc + 1, instruction.modes[0]);
                        self.write_memory(dst, x);
                        self.pc += 2;
                    }
                    None => break State::WaitingForInput,
                },
                OUTPUT => {
                    let x = self.get_parameter(self.pc + 1, instruction.modes[0]);
                    self.outputs.push_back(x);
                    self.pc += 2;
                }
                JUMP_TRUE => self.run_jump_instruction(&instruction, |x| x != 0),
                JUMP_FALSE => self.run_jump_instruction(&instruction, |x| x == 0),
                LESS_THAN => self.run_binary_instruction(&instruction, |x, y| (x < y) as i64),
                EQUAL => self.run_binary_instruction(&instruction, |x, y| (x == y) as i64),
                ADJUST_RELATIVE_BASE => {
                    let x = self.get_parameter(self.pc + 1, instruction.modes[0]);
                    self.rb += x;
                    self.pc += 2;
                }
                HALT => break State::Halted,
                _ => panic!("Unsupported opcode {}", self.ram[self.pc]),
            }
        };
        self.state
    }

    #[inline]
    fn run_binary_instruction(&mut self, instruction: &Instruction, f: fn(i64, i64) -> i64) {
        let in1 = self.get_parameter(self.pc + 1, instruction.modes[0]);
        let in2 = self.get_parameter(self.pc + 2, instruction.modes[1]);
        let dst = self.get_output_addr(self.pc + 3, instruction.modes[2]);
        self.write_memory(dst, f(in1, in2));
        self.pc += 4;
    }

    #[inline]
    fn run_jump_instruction(&mut self, instruction: &Instruction, pred: fn(i64) -> bool) {
        let x = self.get_parameter(self.pc + 1, instruction.modes[0]);
        let addr = self.get_parameter(self.pc + 2, instruction.modes[1]) as usize;
        if pred(x) {
            self.pc = addr;
        } else {
            self.pc += 3;
        }
    }

    #[inline]
    fn decode_instruction(x: i64) -> Instruction {
        Instruction {
            opcode: x % 100,
            modes: [(x / 100) % 10, (x / 1000) % 10, (x / 10000) % 10],
        }
    }

    #[inline]
    fn get_parameter(&self, addr: usize, mode: i64) -> i64 {
        let x = self.read_memory(addr);
        match mode {
            POSITION => self.read_memory(x as usize),
            IMMEDIATE => x,
            RELATIVE => self.read_memory((self.rb + x) as usize),
            _ => panic!("Unsupported mode: {}", mode),
        }
    }

    #[inline]
    fn get_output_addr(&self, addr: usize, mode: i64) -> usize {
        let x = self.read_memory(addr);
        match mode {
            POSITION => x as usize,
            RELATIVE => (self.rb + x) as usize,
            _ => panic!("Unsupported mode: {}", mode),
        }
    }

    #[inline]
    fn read_memory(&self, addr: usize) -> i64 {
        match self.ram.get(addr) {
            Some(val) => *val,
            None => 0,
        }
    }

    #[inline]
    fn write_memory(&mut self, addr: usize, val: i64) {
        if addr >= self.ram.len() {
            self.ram.resize(addr + 1, 0);
        }
        self.ram[addr] = val;
    }
}

pub fn load_program(input_path: &str) -> Vec<i64> {
    std::fs::read_to_string(input_path)
        .unwrap_or_else(|e| panic!("Failed to open {}: {}", input_path, e))
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}
