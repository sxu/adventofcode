const ADD: i32 = 1;
const MUL: i32 = 2;
const HALT: i32 = 99;

pub fn run(ram: &mut Vec<i32>) {
    let mut pc: usize = 0;
    loop {
        match ram[pc] {
            ADD => run_binary_instruction(ram, &mut pc, |x, y| x + y),
            MUL => run_binary_instruction(ram, &mut pc, |x, y| x * y),
            HALT => break,
            _ => panic!("Unsupported opcode {}", ram[pc]),
        }
    }
}

#[inline]
fn run_binary_instruction(ram: &mut Vec<i32>, pc: &mut usize, f: fn(i32, i32) -> i32) {
    let src1 = ram[*pc + 1] as usize;
    let src2 = ram[*pc + 2] as usize;
    let dst = ram[*pc + 3] as usize;
    ram[dst] = f(ram[src1], ram[src2]);
    *pc += 4;
}
