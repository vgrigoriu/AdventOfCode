from input import read_aoc_input


def find_trails(line: int, col: int) -> int:
    if trails[line][col] != -1:
        return trails[line][col]

    start = data[line][col]
    if start == "9":
        # we're done
        trails[line][col] = 1
        return 1

    no_trails = 0
    # up
    if line > 0 and ord(start) + 1 == ord(data[line - 1][col]):
        no_trails += find_trails(line - 1, col)
    # down
    if line < len(data) - 1 and ord(start) + 1 == ord(data[line + 1][col]):
        no_trails += find_trails(line + 1, col)
    # left
    if col > 0 and ord(start) + 1 == ord(data[line][col - 1]):
        no_trails += find_trails(line, col - 1)
    # right
    if col < len(data[0]) - 1 and ord(start) + 1 == ord(data[line][col + 1]):
        no_trails += find_trails(line, col + 1)

    trails[line][col] = no_trails
    return no_trails


def score(line: int, col: int):
    if data[line][col] != "0":
        return 0
    return find_trails(line, col)


def print_trails():
    pass


data = read_aoc_input()
trails = [[-1 for _ in line] for line in data]
sum_of_scores = 0
for line in range(len(data)):
    for col in range(len(data[line])):
        s = score(line, col)
        if s > 0:
            sum_of_scores += score(line, col)
            print_trails()

print(sum_of_scores)
