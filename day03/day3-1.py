#!/usr/bin/python3
import numpy as np

def isValidPos(i, j, n, m):
 
    if (i < 0 or j < 0 or i > n - 1 or j > m - 1):
        return 0
    return 1

def getAdjacent(arr, i, j):
    n = len(arr)
    m = len(arr[0])

    v = []
 
    # Checking for all the possible positions
    if (isValidPos(i - 1, j - 1, n, m)):
        v.append(arr[i - 1][j - 1])
    if (isValidPos(i - 1, j, n, m)):
        v.append(arr[i - 1][j])
    if (isValidPos(i - 1, j + 1, n, m)):
        v.append(arr[i - 1][j + 1])
    if (isValidPos(i, j - 1, n, m)):
        v.append(arr[i][j - 1])
    if (isValidPos(i, j + 1, n, m)):
        v.append(arr[i][j + 1])
    if (isValidPos(i + 1, j - 1, n, m)):
        v.append(arr[i + 1][j - 1])
    if (isValidPos(i + 1, j, n, m)):
        v.append(arr[i + 1][j])
    if (isValidPos(i + 1, j + 1, n, m)):
        v.append(arr[i + 1][j + 1])
 
    # Returning the vector
    return v
"""
with open("day3-input.txt", "r") as f:
    result = 0
    arr = []
    last = False
    for line in f.readlines():
        arr.append(line[:-1])
    arr = np.array([list(line) for line in arr])
    
    for iy, ix in np.ndindex(arr.shape):
        if arr[iy, ix].isnumeric():
            num = arr[iy, ix]
            y, x = iy, ix
            while (x+1 < arr.shape[1]) and arr[y, x+1].isnumeric():
                num += arr[y, x+1]
                rem = len(num)-1
                for r in range(rem):
                    sur = []
                    sur += getAdjacent(arr, iy, ix+r)
                    print(sur)
                    sur = sur[(sur != '.')]
                    sur = sur[(sur!="0") & (sur!="1") & (sur!="2") & (sur!="3") & (sur!="4") & (sur!="5") & (sur!="6") & (sur!="7") & (sur!="8") & (sur!="9")]
                    if sur.size > 0 and rem == 0:
                        result += int(num)
    print(result)
"""
def check_surroundings(arr, y, x, len):
    check = []
    if y > 0:
        print(check += arr[y-1][x-1:x+len+1])

with open("day3-input.txt", "r") as f:
    arr = [list(line[:-1]) for line in f.readlines()]
    result = 0
    
    for y in range(len(arr)):
        for x in range(len(arr[0])):
            if arr[y][x].isnumeric():
                num = int(arr[y][x])
                numlen = 1
                #print(f"digit found: {arr[y][x]}")
                if (x+1 < len(arr[0])) and arr[y][x+1].isnumeric():
                    num = num*10
                    numlen += 1
                    if (x+2 < len(arr[0])) and arr[y][x+2].isnumeric():
                        num = num*10
                        numlen += 1
                if x > 0 or !(arr[y][x-1].isnumeric()):
                    xrange = range(x-1, x+numlen)
                    valid = check_surroundings(arr, y, x, numlen)
                print(num)
                if valid:
                    result += num
                    valid = False
    print(result)
