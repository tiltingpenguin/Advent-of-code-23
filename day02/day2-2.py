#!/usr/bin/python3
import re

re_color = r'(\d+)\s+(\w+)'

result = 0

with open("day2-input.txt", "r") as f:
    for line in f.readlines():
        line = line[:-1]

        min_red = 0
        min_green = 0
        min_blue = 0

        game, parts = line.split(":", 1)
        rounds = parts.split(";")

        for round in rounds:
            counts = re.findall(re_color, round)
            matches = [(int(num), color) for num, color in counts]

            for x in matches:
                if x[1] == "red":
                    if x[0] > min_red:
                        min_red = x[0]
                if x[1] == "green":
                    if x[0] > min_green:
                        min_green = x[0]
                if x[1] == "blue":
                    if x[0] > min_blue:
                        min_blue = x[0]
        set = min_red * min_green * min_blue
        result += set
    print(result)