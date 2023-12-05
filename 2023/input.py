def read_input(day: int, debug: bool = False) -> list[str]:
    # read all the lines
    path = f'input/day{day:02}.txt'
    if debug:
        path = f'input/day{day:02}_debug.txt'
    with open(path) as f:
        lines = f.readlines()
        return [line.strip() for line in lines]


def read_input_as_string(day: int, debug: bool = False) -> str:
    # read all the lines
    path = f'input/day{day:02}.txt'
    if debug:
        path = f'input/day{day:02}_debug.txt'
    with open(path) as f:
        return f.read()
