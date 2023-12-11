import sys

from input import read_input

day = 8


def solve_part_1():
    puzzle_input = read_input(day)
    instructions = puzzle_input[0]
    links = puzzle_input[2:]
    lefts = {}
    rights = {}
    for link in links:
        # FLR = (SXT, CRV)
        node, left_and_right = link.split(" = ")
        left, right = left_and_right[1:-1].split(", ")
        lefts[node] = left
        rights[node] = right

    current_node = 'AAA'
    steps = 0
    for i in range(sys.maxsize):
        instruction = instructions[i % len(instructions)]
        if instruction == 'L':
            current_node = lefts[current_node]
        elif instruction == 'R':
            current_node = rights[current_node]
        steps += 1
        if current_node == 'ZZZ':
            return steps


def solve_part_2():
    puzzle_input = read_input(day)
    return puzzle_input


if __name__ == '__main__':
    print(solve_part_1())
    print(solve_part_2())
