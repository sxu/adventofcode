use crate::intcode;
use std::collections::VecDeque;

pub fn day9(input_path: &str) {
    let rom = intcode::load_program(input_path);

    let ram = rom.clone();
    let mut computer = intcode::Computer::new(ram);
    assert_eq!(
        computer.run_with(&mut VecDeque::from(vec![1])),
        intcode::State::Halted
    );
    assert_eq!(computer.outputs.len(), 1);
    assert_eq!(computer.outputs[0], 3429606717);

    let ram = rom.clone();
    let mut computer = intcode::Computer::new(ram);
    assert_eq!(
        computer.run_with(&mut VecDeque::from(vec![2])),
        intcode::State::Halted
    );
    assert_eq!(computer.outputs.len(), 1);
    assert_eq!(computer.outputs[0], 33679);
}
