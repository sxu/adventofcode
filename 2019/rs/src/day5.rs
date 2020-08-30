use crate::intcode;
use std::collections::VecDeque;

pub fn day5(input_path: &str) {
    let rom = intcode::load_program(input_path);

    let ram = rom.clone();
    let mut computer = intcode::Computer::new(ram);
    let mut inputs = VecDeque::from(vec![1]);
    computer.run_with(&mut inputs);
    for i in 0..(computer.outputs.len() - 1) {
        assert_eq!(computer.outputs[i], 0);
    }
    assert_eq!(*computer.outputs.back().unwrap(), 13285749);

    let ram = rom.clone();
    let mut computer = intcode::Computer::new(ram);
    let mut inputs = VecDeque::from(vec![5]);
    computer.run_with(&mut inputs);
    assert_eq!(computer.outputs.len(), 1);
    assert_eq!(computer.outputs[0], 5000972);
}
