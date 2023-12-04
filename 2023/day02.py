import re
from dataclasses import dataclass

from input import read_input


@dataclass(frozen=True)
class Bag:
    red: int
    green: int
    blue: int


@dataclass(frozen=True)
class Game:
    id: int
    maxRed: int
    maxGreen: int
    maxBlue: int


def find_id(game: str) -> int:
    regex = r"Game (\d+):"
    matches = re.findall(regex, game)
    return int(matches[0])


def find_max(game_line: str, color: str):
    regex = f"(\d+) {color}"
    matches = re.findall(regex, game_line)
    return max([int(match) for match in matches])


# Game 1: 4 green, 2 blue; 1 red, 1 blue, 4 green; 3 green, 4 blue, 1 red; 7 green, 2 blue, 4 red; 3 red, 7 green; 3
# red, 3 green
def parse_game(game_line: str) -> Game:
    id = find_id(game_line)
    max_red = find_max(game_line, "red")
    max_green = find_max(game_line, "green")
    max_blue = find_max(game_line, "blue")
    return Game(id, max_red, max_green, max_blue)


def is_possible(game: Game, bag: Bag) -> bool:
    return game.maxRed <= bag.red and game.maxGreen <= bag.green and game.maxBlue <= bag.blue


def solve_day_02_part_1():
    input = read_input(2)
    all_games = [parse_game(line) for line in input]
    possible_games = [game for game in all_games if is_possible(game, Bag(12, 13, 14))]
    return sum([game.id for game in possible_games])


def power(game: Game) -> int:
    return game.maxRed * game.maxGreen * game.maxBlue


def solve_day_02_part_2():
    input = read_input(2)
    all_games = [parse_game(line) for line in input]
    return sum([power(game) for game in all_games])


if __name__ == '__main__':
    print(solve_day_02_part_1())
    print(solve_day_02_part_2())
