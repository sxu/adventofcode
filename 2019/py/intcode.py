#!/usr/bin/env python3

<<<<<<< HEAD

=======
>>>>>>> 1c222b6f7a72b69af8115d7c54c295da55c97f1d
def run(ram, pc):
    while True:
        opcode = ram[pc]
        if opcode == 1:
            p1 = ram[pc + 1]
            p2 = ram[pc + 2]
            p3 = ram[pc + 3]
            ram[p3] = ram[p1] + ram[p2]
            pc += 4
        elif opcode == 2:
            p1 = ram[pc + 1]
            p2 = ram[pc + 2]
            p3 = ram[pc + 3]
            ram[p3] = ram[p1] * ram[p2]
            pc += 4
        elif opcode == 99:
            return
