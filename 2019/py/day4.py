#!/usr/bin/env python3

import itertools


def day4(input_path):
    with open(input_path) as f:
        min_val, max_val = map(int, f.readline().split("-"))

    part1 = 0
    part2 = 0
    for x in range(min_val, max_val + 1):
        x = str(x)
        if not is_non_decreasing(x):
            continue
        if has_repeats(x):
            part1 += 1
            if has_exact_repeats(x):
                part2 += 1
    assert part1 == 1955
    assert part2 == 1319


def is_non_decreasing(x):
    i1, i2 = itertools.tee(x)
    next(i2, None)
    for a, b in zip(i1, i2):
        if a > b:
            return False
    return True


def has_repeats(x):
    for i in range(len(x) - 1):
        if x[i] == x[i + 1]:
            return True
    return False


def has_exact_repeats(x):
    i = 0
    while i < len(x) - 1:
        val = x[i]
        if val != x[i + 1]:
            i += 1
            continue
        if i + 1 == len(x) - 1 or x[i + 2] != val:
            return True
        while i < len(x) and x[i] == val:
            i += 1
    return False
