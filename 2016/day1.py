from direction import Direction
from input import read_aoc_input


def parse_instruction(instr: str):
    return instr[0], int(instr[1:])


def parse_instructions(line: str):
    return [parse_instruction(instr) for instr in line.split(", ")]


input = read_aoc_input(parse_instructions)

lat = 0
long = 0
dir = Direction()

def walk(blocks: int):
    global lat, long
    if str(dir) == "N":
        lat += blocks
    elif str(dir) == "S":
        lat -= blocks
    elif str(dir) == "E":
        long += blocks
    elif str(dir) == "W":
        long -= blocks

for side, blocks in input:
    dir.turn(side)
    walk(blocks)

print(abs(lat) + abs(long))
