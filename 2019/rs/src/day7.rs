use crate::intcode;
use itertools::Itertools;
use std::collections::VecDeque;

pub fn day7(input_path: &str) {
    let rom = intcode::load_program(input_path);

    let mut max_thrust = i32::MIN;
    for phases in (0..5).permutations(5) {
        let mut amps = set_up_amps(&rom, &phases);
        let mut first_signal = VecDeque::from(vec![0]);
        assert_eq!(amps[0].run_with(&mut first_signal), intcode::State::Halted);
        for i in 1..5 {
            unsafe {
                let prev = &mut *(amps.get_unchecked_mut(i - 1) as *mut intcode::Computer);
                let this = &mut *(amps.get_unchecked_mut(i) as *mut intcode::Computer);
                assert_eq!(this.run_with(&mut prev.outputs), intcode::State::Halted);
            }
        }
        if amps[4].outputs[0] > max_thrust {
            max_thrust = amps[4].outputs[0];
        }
    }
    assert_eq!(max_thrust, 206580);

    let mut max_thrust = i32::MIN;
    for phases in (5..10).permutations(5) {
        let mut amps = set_up_amps(&rom, &phases);
        amps[4].outputs.push_back(0);
        loop {
            for i in 0..5 {
                unsafe {
                    let prev =
                        &mut *(amps.get_unchecked_mut((i + 5 - 1) % 5) as *mut intcode::Computer);
                    let this = &mut *(amps.get_unchecked_mut(i) as *mut intcode::Computer);
                    this.run_with(&mut prev.outputs);
                }
            }
            if amps[4].state == intcode::State::Halted {
                break;
            }
        }
        if amps[4].outputs[0] > max_thrust {
            max_thrust = amps[4].outputs[0];
        }
    }
    assert_eq!(max_thrust, 2299406);
}

fn set_up_amps(rom: &Vec<i32>, phases: &[i32]) -> [intcode::Computer; 5] {
    let mut amps = [
        intcode::Computer::new(rom.clone()),
        intcode::Computer::new(rom.clone()),
        intcode::Computer::new(rom.clone()),
        intcode::Computer::new(rom.clone()),
        intcode::Computer::new(rom.clone()),
    ];
    let mut phase_input = VecDeque::new();
    for i in 0..5 {
        phase_input.push_back(phases[i]);
        amps[i].run_with(&mut phase_input);
    }
    amps
}
