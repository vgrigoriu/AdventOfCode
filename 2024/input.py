import inspect
from typing import Callable


def read_aoc_input[T](line_parser: Callable[[str], T]) -> list[T]:
    """
    Reads the input file for the current day and returns a list of parsed lines.
    Assumes that the caller is in a file named dayX.py where X is the day number.
    """
    caller_filename = inspect.stack()[1].filename
    day = int(caller_filename.split("/")[-1].split(".")[0][3:])
    with open(f"input/{day}.in") as f:
        return [line_parser(line) for line in f.readlines()]
