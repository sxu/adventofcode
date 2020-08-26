#!/usr/bin/env python3

import sys
from day1 import day1
from day2 import day2
from day3 import day3
from day4 import day4

# from day18 import day18


def main():
    days = [day1, day2, day3, day4]

    if len(sys.argv) > 1:
        day = int(sys.argv[1])
        days[day - 1](f"../input{day}")
    else:
        for i, day in enumerate(days):
            print(f"Day {i + 1}...", end=" ")
            day(f"../input{i + 1}")
            print("OK")


if __name__ == "__main__":
    main()
