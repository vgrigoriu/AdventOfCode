import math
from dataclasses import dataclass
from functools import reduce
from math import sqrt

from input import read_input

day = 6


@dataclass(frozen=True)
class Race:
    time: int
    distance_to_beat: int


def times_to_press(race: Race) -> (float, float):
    min_t = (race.time - sqrt(race.time * race.time - 4 * race.distance_to_beat)) / 2
    max_t = (race.time + sqrt(race.time * race.time - 4 * race.distance_to_beat)) / 2
    return min_t, max_t


def ways_to_win(min_t: float, max_t: float) -> int:
    return math.floor(max_t) - math.ceil(min_t) + 1

def solve_part_1():
    puzzle_input = read_input(day)
    # Time:        45     98     83     73
    times = [int(time) for time in puzzle_input[0].split(":")[1].strip().split()]
    distances = [int(distance) for distance in puzzle_input[1].split(":")[1].strip().split()]
    races = [Race(time, distance) for time, distance in zip(times, distances)]
    times = [times_to_press(race) for race in races]
    ways = [ways_to_win(min_t, max_t) for min_t, max_t in times]
    return reduce(lambda x, y: x * y, ways)


def solve_part_2():
    puzzle_input = read_input(day)
    # Time:        45     98     83     73
    time = int(puzzle_input[0].split(":")[1].strip().replace(" ", ""))
    distance = int(puzzle_input[1].split(":")[1].strip().replace(" ", ""))
    race = Race(time, distance)
    min_t, max_t = times_to_press(race)
    ways = ways_to_win(min_t, max_t)
    return ways


if __name__ == '__main__':
    print(solve_part_1())
    print(solve_part_2())
