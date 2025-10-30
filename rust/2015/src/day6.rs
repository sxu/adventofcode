use crate::lights::Grid;
use itertools::Itertools;

#[derive(Debug)]
enum OpCode {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug)]
struct Instruction {
    op_code: OpCode,
    upper_left: (usize, usize),
    lower_right: (usize, usize),
}

fn parse_instruction(instruction: &str) -> Instruction {
    let mut it = instruction.split(' ');
    let op_code = match it.next().unwrap() {
        "turn" => match it.next().unwrap() {
            "on" => OpCode::TurnOn,
            "off" => OpCode::TurnOff,
            _ => panic!("Invalid instruction {instruction}"),
        },
        "toggle" => OpCode::Toggle,
        _ => panic!("Invalid instruction {instruction}"),
    };
    let upper_left = it.next().unwrap();
    assert_eq!(it.next().unwrap(), "through");
    let lower_right = it.next().unwrap();
    assert_eq!(it.next(), None);

    let upper_left = upper_left
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    let lower_right = lower_right
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();

    Instruction {
        op_code,
        upper_left,
        lower_right,
    }
}

pub fn solve(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Failed to read file.");
    let mut binary_grid = Grid::<bool>::new(1000, 1000, false);
    for ins in input.lines().map(parse_instruction) {
        let f: fn(&bool) -> bool = match ins.op_code {
            OpCode::TurnOn => |_| true,
            OpCode::TurnOff => |_| false,
            OpCode::Toggle => |b: &bool| !b,
        };
        binary_grid.apply(ins.upper_left, ins.lower_right, f)
    }
    let mut cnt: usize = 0;
    for x in binary_grid.data() {
        if *x {
            cnt += 1;
        }
    }
    assert_eq!(cnt, 569999);

    let mut analog_grid = Grid::<i32>::new(1000, 1000, 0);
    for ins in input.lines().map(parse_instruction) {
        let f: fn(&i32) -> i32 = match ins.op_code {
            OpCode::TurnOn => |x| x + 1,
            OpCode::TurnOff => |x| std::cmp::max(x - 1, 0),
            OpCode::Toggle => |x| x + 2,
        };
        analog_grid.apply(ins.upper_left, ins.lower_right, f)
    }
    let mut total: i32 = 0;
    for x in analog_grid.data() {
        total += x;
    }
    assert_eq!(total, 17836115);
}
