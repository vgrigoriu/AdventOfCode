import re
from dataclasses import dataclass

from input import read_aoc_input


# rect 2x1
@dataclass
class Rect:
    cols: int
    rows: int


# rotate row y=0 by 5
@dataclass
class RotateRow:
    row: int
    by: int


# rotate column x=0 by 1
@dataclass
class RotateColumn:
    col: int
    by: int


type Instruction = Rect | RotateRow | RotateColumn


def parse_instruction(line: str) -> Instruction:
    match_rect = re.match(r"rect (\d+)x(\d+)", line)
    if match_rect:
        return Rect(cols=int(match_rect[1]), rows=int(match_rect[2]))

    match_rotate_row = re.match(r"rotate row y=(\d+) by (\d+)", line)
    if match_rotate_row:
        return RotateRow(row=int(match_rotate_row[1]), by=int(match_rotate_row[2]))

    match_rotate_column = re.match(r"rotate column x=(\d+) by (\d+)", line)
    if match_rotate_column:
        return RotateColumn(
            col=int(match_rotate_column[1]), by=int(match_rotate_column[2])
        )


def apply_instruction(instr: Instruction) -> None:
    match instr:
        case Rect(cols, rows):
            for row_no in range(rows):
                for col_no in range(cols):
                    screen[row_no][col_no] = "#"
        case RotateRow(row, by):
            screen[row] = screen[row][-by:] + screen[row][:-by]
        case RotateColumn(col, by):
            column = [row[col] for row in screen]
            column = column[-by:] + column[:-by]
            for row_no in range(len(screen)):
                screen[row_no][col] = column[row_no]
        case _:
            raise ValueError(f"Don't know how to apply {instr}")


def print_screen() -> None:
    for pixel_row in screen:
        print("".join(pixel_row))


instructions = read_aoc_input(parse_instruction)

screen = [["." for _ in range(50)] for _ in range(6)]

for instr in instructions:
    apply_instruction(instr)

part1 = len([pixel for line in screen for pixel in line if pixel == "#"])
print(part1)
print_screen()
