#!/usr/bin/python3

nums = {
    "one": "1",
    "two": "2",
    "three": "3",
    "four": "4",
    "five": "5",
    "six": "6",
    "seven": "7",
    "eight": "8",
    "nine": "9",
}

result = 0

with open("day1-1-input.txt") as f:
    lines = f.readlines()
    for line in lines:
        found = []
        for char in line:
            if char.isnumeric():
                found.append((char, line.index(char)))
        for num in nums.keys():
            index = line.find(num)
            if index != -1:
                found.append((nums[num], index))
            last_index = line.rfind(num)
            if last_index != -1 and (last_index != index):
                found.append((nums[num], index + len(num)))
        found.sort(key = lambda a: a[1])
        first = found[0][0]
        second = found[-1][0]
        result += int(first + second)
    print(result)