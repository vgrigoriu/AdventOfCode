from typing import Callable


def read[T](day: int, line_parser: Callable[[str], T]) -> list[T]:
    with open(f"input/{day:02d}.in") as f:
        return [line_parser(line) for line in f.readlines()]