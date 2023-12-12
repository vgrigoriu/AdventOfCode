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
    instructions = puzzle_input[0]
    links = puzzle_input[2:]
    lefts = {}
    rights = {}
    start_nodes = []
    for link in links:
        # FLR = (SXT, CRV)
        node, left_and_right = link.split(" = ")
        left, right = left_and_right[1:-1].split(", ")
        lefts[node] = left
        rights[node] = right
        if node.endswith('A'):
            start_nodes.append(node)

    print(start_nodes)

    for node_index in range(len(start_nodes)):
        visited = set()
        current_node = start_nodes[node_index]
        for i in range(sys.maxsize):
            instr_index = i % len(instructions)
            if current_node.endswith('Z'):
                print("reached Z at ", i, instr_index, current_node)
            instruction = instructions[instr_index]
            #print(f"step {i} at {current_node}, going {instruction}")
            if (current_node, instr_index) in visited:
                print("repeats at ", instr_index, current_node, instruction)
                break
            visited.add((current_node, instr_index))
            current_node = lefts[current_node] if instruction == 'L' else rights[current_node]

    # current_nodes = start_nodes
    # steps = 0
    # for i in range(sys.maxsize):
    #     instruction = instructions[i % len(instructions)]
    #     steps += 1
    #     if instruction == 'L':
    #         next_nodes = [lefts[current_node] for current_node in current_nodes]
    #     elif instruction == 'R':
    #         next_nodes = [rights[current_node] for current_node in current_nodes]
    #     no_of_zs = len([node for node in next_nodes if node.endswith('Z')])
    #     if no_of_zs == len(next_nodes):
    #         return steps
    #     if no_of_zs > 0:
    #         print(i, no_of_zs, next_nodes)
    #     current_nodes = next_nodes


if __name__ == '__main__':
    print(solve_part_1())
    print(solve_part_2())
