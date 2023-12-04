def read_input(day: int) -> list[str]:
    # read all the lines
    with open(f'input/day{day:02}.txt') as f:
        lines = f.readlines()
        return [line.strip() for line in lines]
