import re

def part1():
    with open("input.txt", "r") as file:
        text = file.read()
        sum = 0
        for factor0, factor1 in re.findall("mul\(([0-9]+),([0-9]+)\)", text):
            if len(factor0) > 3:
                continue
            if len(factor1) > 3:
                continue
            sum += int(factor0) * int(factor1)
        print(sum)

def part2():
    with open("input.txt", "r") as file:
        text = file.read()
        sum = 0
        mul_enabled = True
        for factor0, factor1, do, dont in re.findall("mul\(([0-9]+),([0-9]+)\)|(do\(\))|(don't\(\))", text):
            if len(do) > 0:
                mul_enabled = True
                continue
            if len(dont) > 0:
                mul_enabled = False
                continue
            if mul_enabled:
                sum += int(factor0) * int(factor1)
        print(sum)

if __name__ == '__main__':
    part1()
    part2()

