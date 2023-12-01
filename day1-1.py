#!/usr/bin/python3

result = 0

with open("day1-1-input.txt") as f:
    lines = f.readlines()
    for line in lines:
        for char in line:
            if char.isnumeric():
                first = char
                break
        for char in reversed(line):
            if char.isnumeric():
                second = char
                break
        result += int(first + second)
    print(result)