import re
from collections import defaultdict

from input import read_input
from collections.abc import Iterator
from dataclasses import dataclass


@dataclass(frozen=True)
class Number:
    repr: str
    line: int
    begin_column: int

    def end_column(self) -> int:
        return self.begin_column + len(self.repr)


@dataclass(frozen=True)
class Gear:
    line: int
    column: int


class Schematic:
    def __init__(self, lines: list[str]):
        self.lines = lines
        self.width = len(lines[0])
        self.height = len(lines)

    def numbers(self) -> Iterator[Number]:
        number_re = "\d+"
        for i in range(self.height):
            for match in re.finditer(number_re, self.lines[i]):
                yield Number(match.group(), i, match.start())

    def is_part_number(self, number: Number) -> bool:
        # must be adjacent to a symbol
        for i in range(number.line - 1, number.line + 2):
            for j in range(number.begin_column - 1, number.end_column() + 1):
                c = self.char_at(i, j)
                if (not c.isdigit()) and c != ".":
                    return True
        return False

    def find_gears_near(self, number: Number) -> Iterator[Gear]:
        for line in range(number.line - 1, number.line + 2):
            for col in range(number.begin_column - 1, number.end_column() + 1):
                c = self.char_at(line, col)
                if c == "*":
                    yield Gear(line, col)

    def char_at(self, i: int, j: int) -> str:
        if i < 0 or i >= self.height:
            return "."
        if j < 0 or j >= self.width:
            return "."
        return self.lines[i][j]

    def __str__(self):
        return f"Schematic(width={self.width}, height={self.height})"


def solve_day_03_part_1():
    input = read_input(3)
    engine = Schematic(input)
    numbers = list(engine.numbers())
    part_numbers = [number for number in numbers if engine.is_part_number(number)]
    return sum([int(number.repr) for number in part_numbers])


def solve_day_03_part_2():
    input = read_input(3)
    engine = Schematic(input)
    numbers = list(engine.numbers())
    gears_to_numbers = defaultdict(list)
    for number in numbers:
        for gear in engine.find_gears_near(number):
            gears_to_numbers[gear].append(number)
    real_gears = [gear for gear in gears_to_numbers if len(gears_to_numbers[gear]) == 2]
    gear_ratios = [int(gears_to_numbers[gear][0].repr) * int(gears_to_numbers[gear][1].repr) for gear in real_gears]
    return sum(gear_ratios)


if __name__ == '__main__':
    print(solve_day_03_part_1())
    print(solve_day_03_part_2())
