from board import Board
from input import read_aoc_input


input = read_aoc_input()

board = Board(["123", "456", "789"], 1, 1)

for line in input:
    for dir in line:
        board.move(dir)
    print(board.current(), end="")
print()

new_board = Board(
    [
        "  1  ",
        " 234 ",
        "56789",
        " ABC ",
        "  D  "
    ],
    2, 0
)

for line in input:
    for dir in line:
        new_board.move(dir)
    print(new_board.current(), end="")
print()