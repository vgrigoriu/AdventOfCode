import hashlib
import queue
import typing


class Coord(typing.NamedTuple):
    x: int
    y: int


class Day17Grid:
    def __init__(
        self,
        passcode: str,
        target: Coord = Coord(3, 3),
        start: Coord = Coord(0, 0),
        size: int = 4,
    ) -> None:
        self._passcode = passcode
        self._target = target
        self._start = start
        self._size = size

    def find_first_path(self) -> str | None:
        paths = queue.Queue()
        # start at the beginning
        paths.put("")

        while not paths.empty():
            current = paths.get()
            open_doors = self.open_doors(current)
            x, y = self.coord(current)
            for door in open_doors:
                match door:
                    case "U" if y > 0:
                        new_path = current + door
                        if self.coord(new_path) == self._target:
                            return new_path
                        paths.put(new_path)
                    case "D" if y < self._size - 1:
                        new_path = current + door
                        if self.coord(new_path) == self._target:
                            return new_path
                        paths.put(new_path)
                    case "L" if x > 0:
                        new_path = current + door
                        if self.coord(new_path) == self._target:
                            return new_path
                        paths.put(new_path)
                    case "R" if x < self._size - 1:
                        new_path = current + door
                        if self.coord(new_path) == self._target:
                            return new_path
                        paths.put(new_path)

        # if we got here, we couldn't find a path
        return None

    def find_longest_path(self) -> str | None:
        paths = queue.Queue()
        # start at the beginning
        paths.put("")
        longest_path = None

        while not paths.empty():
            current = paths.get()
            open_doors = self.open_doors(current)
            x, y = self.coord(current)
            for door in open_doors:
                match door:
                    case "U" if y > 0:
                        new_path = current + door
                        if self.coord(new_path) == self._target:
                            longest_path = new_path
                        else:
                            paths.put(new_path)
                    case "D" if y < self._size - 1:
                        new_path = current + door
                        if self.coord(new_path) == self._target:
                            longest_path = new_path
                        else:
                            paths.put(new_path)
                    case "L" if x > 0:
                        new_path = current + door
                        if self.coord(new_path) == self._target:
                            longest_path = new_path
                        else:
                            paths.put(new_path)
                    case "R" if x < self._size - 1:
                        new_path = current + door
                        if self.coord(new_path) == self._target:
                            longest_path = new_path
                        else:
                            paths.put(new_path)

        return longest_path

    def open_doors(self, path: str) -> list[str]:
        hash = hashlib.md5((self._passcode + path).encode()).hexdigest()[:4]
        return [
            direction
            for direction, ch in zip("UDLR", hash, strict=True)
            if ch in "bcdef"
        ]

    def coord(self, path: str) -> Coord:
        result = self._start
        for direction in path:
            x, y = result
            match direction:
                case "U":
                    result = Coord(x, y - 1)
                case "D":
                    result = Coord(x, y + 1)
                case "L":
                    result = Coord(x - 1, y)
                case "R":
                    result = Coord(x + 1, y)

        return result


def test_open_doors() -> None:
    sut = Day17Grid("hijkl")

    assert sut.open_doors("") == ["U", "D", "L"]
    assert sut.open_doors("D") == ["U", "L", "R"]
    assert sut.open_doors("DR") == []
    assert sut.open_doors("DU") == ["R"]
    assert sut.open_doors("DUR") == []


def test_grid_coord() -> None:
    sut = Day17Grid("abc")
    assert sut.coord("") == Coord(0, 0)
    assert sut.coord("D") == Coord(0, 1)
    assert sut.coord("DRURDRUDDLLDLUURRDULRLDUUDDDRR") == Coord(3, 3)


def test_find_first_path() -> None:
    sut = Day17Grid("hijkl")
    assert sut.find_first_path() is None
    sut = Day17Grid("ihgpwlah")
    assert sut.find_first_path() == "DDRRRD"


grid = Day17Grid("bwnlcvfs")
print(grid.find_first_path())
print(len(grid.find_longest_path()))
