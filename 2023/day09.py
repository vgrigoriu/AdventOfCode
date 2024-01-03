from input import read_input

day = 9


def differences(values: list[int]) -> list[int]:
    return [values[i + 1] - values[i] for i in range(len(values) - 1)]


def all_zeroes(values: list[int]) -> bool:
    return all([value == 0 for value in values])


def generate_sequences(values: list[int]) -> list[list[int]]:
    sequences = []
    current_sequence = values
    while not all_zeroes(current_sequence):
        sequences.append(current_sequence)
        current_sequence = differences(current_sequence)
    return sequences


def fill_last_value(sequence: list[list[int]]) -> int:
    sequence = list(reversed(sequence))
    for i in range(len(sequence)-2, -1, -1):
        sequence[i].append(sequence[i][-1] + sequence[i+1][-1])
    return sequence[0][-1]


def fill_first_value(sequence: list[list[int]]) -> int:
    for i in range(len(sequence)-2, -1, -1):
        sequence[i].insert(0, sequence[i][0] - sequence[i+1][0])
    return sequence[0][0]


def solve_part_1():
    puzzle_input = read_input(day)
    # 10 13 16 21 30 45
    values = [[int(value) for value in line.split()] for line in puzzle_input]
    sequences = [generate_sequences(xs) for xs in values]
    results = [fill_last_value(sequence) for sequence in sequences]
    return sum(results)


def solve_part_2():
    puzzle_input = read_input(day)
    # 10 13 16 21 30 45
    values = [[int(value) for value in line.split()] for line in puzzle_input]
    sequences = [generate_sequences(xs) for xs in values]
    results = [fill_first_value(sequence) for sequence in sequences]
    return sum(results)


if __name__ == '__main__':
    print(solve_part_1())
    print(solve_part_2())
