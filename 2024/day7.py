from input import read_aoc_input


def parse(line: str) -> tuple[int, list[int]]:
    test_value, rest = line.split(": ")
    return int(test_value), [int(n) for n in rest.split(" ")]


def generate_operators(n: int, possible_operators: list[str]) -> list[list[str]]:
    if n == 0:
        return [[]]

    less_operators = generate_operators(n - 1, possible_operators)
    return [ops + [op] for ops in less_operators for op in possible_operators]


def evaluate(numbers: list[int], ops: list[str]) -> int:
    result = numbers[0]
    for i in range(len(ops)):
        if ops[i] == "+":
            result += numbers[i + 1]
        elif ops[i] == "*":
            result *= numbers[i + 1]
        elif ops[i] == "||":
            result = int(str(result) + str(numbers[i + 1]))

    return result


equations = read_aoc_input(parse)

part1 = 0
for test_value, numbers in equations:
    opses = generate_operators(len(numbers) - 1, ["+", "*"])
    for ops in opses:
        if evaluate(numbers, ops) == test_value:
            part1 += test_value
            break

print(part1)

part2 = 0
for test_value, numbers in equations:
    opses = generate_operators(len(numbers) - 1, ["+", "*", "||"])
    for ops in opses:
        if evaluate(numbers, ops) == test_value:
            part2 += test_value
            break

print(part2)
