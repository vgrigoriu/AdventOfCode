from board import Board
from input import read_aoc_input


input = read_aoc_input()

board = Board(["123", "456", "789"], 1, 1)

for line in input:
    for dir in line:
        board.move(dir)
    print(board.current(), end="")
print()