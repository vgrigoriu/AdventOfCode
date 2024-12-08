from board import Board
from input import read_aoc_input

input = read_aoc_input()

board = Board(["123", "456", "789"])
pos = board.pos(1, 1)

for line in input:
    for dir in line:
        pos.move(dir)
    print(board[pos], end="")
print()

new_board = Board(
    [
        "  1  ",
        " 234 ",
        "56789",
        " ABC ",
        "  D  ",
    ],
)  # fmt: skip
pos = new_board.pos(2, 0)

for line in input:
    for dir in line:
        pos.move(dir)
    print(new_board[pos], end="")
print()
