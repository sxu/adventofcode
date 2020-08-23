#!/usr/bin/env python3

<<<<<<< HEAD

=======
>>>>>>> 1c222b6f7a72b69af8115d7c54c295da55c97f1d
def day1(input_path):
    with open(input_path) as f:
        masses = list(map(lambda x: int(x), f))

    assert sum(map(simple_fuel_calc, masses)) == 3455717
    assert sum(map(complex_fuel_calc, masses)) == 5180690

<<<<<<< HEAD

def simple_fuel_calc(mass):
    return mass // 3 - 2


=======
def simple_fuel_calc(mass):
    return mass // 3 - 2

>>>>>>> 1c222b6f7a72b69af8115d7c54c295da55c97f1d
def complex_fuel_calc(mass):
    total = 0
    while mass > 0:
        fuel = max(mass // 3 - 2, 0)
        total += fuel
        mass = fuel
    return total
