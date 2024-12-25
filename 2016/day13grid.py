import dataclasses
import functools
from collections.abc import Iterable
from typing import Any

from grid import Coord, Grid


class Day13Grid(Grid):
    def __init__(self, fav_num: int) -> None:
        self._fav_num = fav_num
        self._max_x = 0
        self._max_y = 0

    @functools.cache
    def __getitem__(self, coord: Coord) -> Any:
        x, y = dataclasses.astuple(coord)

        # Did we go beyond?
        self._max_x = max(x, self._max_x)
        self._max_y = max(y, self._max_y)

        value: int = x * x + 3 * x + 2 * x * y + y + y * y
        value += self._fav_num
        num_bits = value.bit_count()
        if num_bits % 2 == 0:
            return "."
        else:
            return "#"

    def __iter__(self):
        for y in range(self._max_y + 1):
            yield self._row_iter(y)

    def _row_iter(self, y: int) -> Iterable[str]:
        for x in range(self._max_x + 1):
            yield self[Coord(x, y)]

    def is_visitable(self, coord: Coord) -> bool:
        return coord.x >= 0 and coord.y >= 0 and self[coord] == "."


def test_origin() -> None:
    sut = Day13Grid(10)

    result = sut[Coord(0, 0)]

    assert result == "."


def test_1_0() -> None:
    sut = Day13Grid(10)

    result = sut[Coord(1, 0)]

    assert result == "#"


def test_iterator_up_to_9_6() -> None:
    sut = Day13Grid(10)

    # extend gord up to (9, 6)
    assert sut[Coord(9, 6)] == "#"

    result = ["".join(row) for row in sut]

    assert result == [
        ".#.####.##",
        "..#..#...#",
        "#....##...",
        "###.#.###.",
        ".##..#..#.",
        "..##....#.",
        "#...##.###",
    ]


def test_find_path_to_7_4() -> None:
    sut = Day13Grid(10)

    result = sut.shortest_path(Coord(1, 1), Coord(7, 4))

    assert result == 11


def test_part_1() -> None:
    grid = Day13Grid(1352)
    result = grid.shortest_path(Coord(1, 1), Coord(31, 39))

    assert result == 90


def test_visitable_max_steps_0() -> None:
    sut = Day13Grid(10)
    result = sut.how_many_visited(Coord(1, 1), 0)
    assert result == 1


def test_visitable_max_steps_1() -> None:
    sut = Day13Grid(10)
    result = sut.how_many_visited(Coord(1, 1), 1)
    assert result == 3


def test_visitable_max_steps_2() -> None:
    sut = Day13Grid(10)
    result = sut.how_many_visited(Coord(1, 1), 2)
    assert result == 5


def test_part_2() -> None:
    grid = Day13Grid(1352)
    result = grid.how_many_visited(Coord(1, 1), 50)

    assert result == 135
