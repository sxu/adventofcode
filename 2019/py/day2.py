#!/usr/bin/env python3

import numpy as np

import intcode

def day2(input_path):
    with open(input_path) as f:
        rom = np.ascontiguousarray(list(map(int, f.read().split(","))))

    ram = rom.copy()
    ram[1] = 12
    ram[2] = 2
    intcode.run(ram, 0)
    assert ram[0] == 5290681
    assert search(rom) == 5741

def search(rom):
    for noun in range(100):
        for verb in range(100):
            ram = rom.copy()
            ram[1] = noun
            ram[2] = verb
            intcode.run(ram, 0)
            if ram[0] == 19690720:
                return noun * 100 + verb

    raise RuntimeError("Not found")
