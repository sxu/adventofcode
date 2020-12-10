use itertools::Itertools;
use std::collections::HashSet;

use crate::utils;

#[derive(Debug)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

use Instruction::*;

fn parse_instruction(line: &str) -> Instruction {
    let (op, arg) = line.split(' ').next_tuple().unwrap();
    let mut val = arg[1..].parse::<i32>().unwrap();
    if arg.chars().nth(0).unwrap() == '-' {
        val = -val;
    }
    match op {
        "acc" => Acc(val),
        "jmp" => Jmp(val),
        "nop" => Nop(val),
        _ => panic!("Unknown instruction: {}", op),
    }
}

fn run(program: &Vec<Instruction>) -> (i32, bool) {
    let mut acc: i32 = 0;
    let mut pc: i32 = 0;
    let mut executed: HashSet<i32> = HashSet::new();
    loop {
        if (pc as usize) == program.len() {
            return (acc, true);
        }
        if executed.contains(&pc) {
            return (acc, false);
        }
        executed.insert(pc);
        match program[pc as usize] {
            Acc(val) => {
                acc += val;
                pc += 1;
            }
            Jmp(val) => {
                pc += val;
            }
            _ => {
                pc += 1;
            }
        }
    }
}

pub fn day8(input_path: &str) {
    let mut program: Vec<Instruction> = utils::input_lines(input_path)
        .map(|l| parse_instruction(&l))
        .collect();

    let (acc, _) = run(&program);
    assert_eq!(acc, 1217);

    let mut terminated_acc: Option<i32> = None;
    for i in 0..program.len() {
        let mut tmp: Instruction;
        match program[i] {
            Jmp(val) => {
                tmp = Nop(val);
                std::mem::swap(&mut program[i], &mut tmp);
            }
            Nop(val) => {
                tmp = Jmp(val);
                std::mem::swap(&mut program[i], &mut tmp);
            }
            _ => {
                continue;
            }
        }
        let (acc, terminated) = run(&program);
        if terminated {
            terminated_acc = Some(acc);
            break;
        }
        std::mem::swap(&mut program[i], &mut tmp);
    }
    assert_eq!(terminated_acc, Some(501));
}
