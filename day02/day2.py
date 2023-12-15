#!/usr/bin/python3
import re

re_color = r'(\d+)\s+(\w+)'

result = 0
max_red = 12
max_green = 13
max_blue = 14

with open("day2-input.txt", "r") as f:
    for line in f.readlines():
        line = line[:-1]
        impossible = False
        game, parts = line.split(":", 1)
        colors = parts.split(";")
        for entry in colors:
            counts = re.findall(re_color, entry)
            matches = [(int(num), color) for num, color in counts]
            for x in matches:
                if x[1] == "red" and x[0] > max_red:
                    impossible = True
                if x[1] == "green" and x[0] > max_green:
                    impossible = True
                if x[1] == "blue" and x[0] > max_blue:
                    impossible = True
            if impossible == True:
                break
        if impossible == False:
            game = game[5:]
            result += int(game)
    print(result)