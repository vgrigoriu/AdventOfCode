import dataclasses
from abc import ABC, abstractmethod
from collections.abc import Iterable
from dataclasses import dataclass
from queue import Queue
from typing import Any


@dataclass(frozen=True)
class Coord:
    x: int
    y: int


class Grid(ABC):
    @abstractmethod
    def __getitem__(self, coord: Coord) -> str: ...

    @abstractmethod
    def __iter__(self) -> Iterable[Iterable[Any]]:
        """return an iterator that returns one iterator per row of the grid"""
        ...

    @abstractmethod
    def is_visitable(self, coord: Coord) -> bool: ...

    def shortest_path(self, start: Coord, target: Coord) -> int | None:
        visited = set()
        to_visit = Queue()
        to_visit.put((start, 0))

        while not to_visit.empty():
            coord, distance = to_visit.get()
            if coord in visited:
                continue

            visited.add(coord)

            neighbors = [
                dataclasses.replace(coord, x=coord.x - 1),
                dataclasses.replace(coord, x=coord.x + 1),
                dataclasses.replace(coord, y=coord.y - 1),
                dataclasses.replace(coord, y=coord.y + 1),
            ]

            for neighbor in neighbors:
                if neighbor == target:
                    return distance + 1
                if self.is_visitable(neighbor):
                    to_visit.put((neighbor, distance + 1))

        # no path found
        return None

    def how_many_visited(self, start: Coord, max_steps: int) -> int:
        visited = {}
        to_visit = Queue()
        to_visit.put((start, 0))

        while not to_visit.empty():
            coord, distance = to_visit.get()
            if distance == max_steps + 1:
                return len(visited)
            if coord in visited:
                continue

            visited[coord] = distance

            neighbors = [
                dataclasses.replace(coord, x=coord.x - 1),
                dataclasses.replace(coord, x=coord.x + 1),
                dataclasses.replace(coord, y=coord.y - 1),
                dataclasses.replace(coord, y=coord.y + 1),
            ]

            for neighbor in neighbors:
                if self.is_visitable(neighbor):
                    to_visit.put((neighbor, distance + 1))
