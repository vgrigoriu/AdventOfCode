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
    if dir == "N":
        lat += blocks
    elif dir == "S":
        lat -= blocks
    elif dir == "E":
        long += blocks
    elif dir == "W":
        long -= blocks


for side, blocks in input:
    dir.turn(side)
    walk(blocks)

print(abs(lat) + abs(long))

lat = 0
long = 0
dir = Direction()
visited = [(0, 0)]


def walk_memo(blocks: int) -> bool:
    global lat, long
    d_lat, d_long = 0, 0
    if dir == "N":
        d_lat = 1
    elif dir == "S":
        d_lat = -1
    elif dir == "E":
        d_long = 1
    elif dir == "W":
        d_long = -1

    for _ in range(blocks):
        lat += d_lat
        long += d_long
        if (lat, long) in visited:
            return True
        visited.append((lat, long))


for side, blocks in input:
    dir.turn(side)
    found = walk_memo(blocks)
    if found:
        break

print(abs(lat) + abs(long))
