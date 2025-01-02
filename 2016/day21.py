import re
from dataclasses import dataclass

from input import read_aoc_input


@dataclass
class Reverse:
    from_inclusive: int
    to_inclusive: int

    def apply(self, password: str) -> str:
        result = list(password)
        from_index = self.from_inclusive - 1 if self.from_inclusive > 0 else None
        result[self.from_inclusive:self.to_inclusive + 1] = result[self.to_inclusive:from_index:-1]

        return "".join(result)

    def unapply(self, password: str) -> str:
        return self.apply(password)


def test_reverse() -> None:
    sut = Reverse(1, 3)
    assert sut.apply("abcde") == "adcbe"
    assert sut.unapply("adcbe") == "abcde"

    sut = Reverse(from_inclusive=0, to_inclusive=3)
    assert sut.apply("gabfecdh") == "fbagecdh"
    assert sut.unapply("fbagecdh") == "gabfecdh"


@dataclass
class RotateByLetter:
    letter: str

    def apply(self, password: str) -> str:
        result = list(password)
        index = result.index(self.letter)
        rotation = index + 1 + (1 if index >= 4 else 0)
        rotation = rotation % len(result)
        return "".join(result[-rotation:] + result[:-rotation])

    def unapply(self, password: str) -> str:
        for i in range(len(password)):
            result = RotateLeft(i).apply(password)
            if self.apply(result) == password:
                return result


def test_rotate_by_letter() -> None:
    sut = RotateByLetter("X")

    assert sut.apply("abXdefg") == "efgabXd"
    assert sut.apply("abcdeXg") == "abcdeXg"
    assert sut.apply("abcdeXghij") == "deXghijabc"
    assert sut.apply("abcdefX") == "Xabcdef"


def test_rotate_by_letter_unapply() -> None:
    sut = RotateByLetter("X")

    assert sut.unapply("efgabXd") == "abXdefg"
    assert sut.unapply("abcdeXg") == "abcdeXg"
    assert sut.unapply("deXghijabc") == "abcdeXghij"
    assert sut.apply("Xabcdef") == "abcdefX"


@dataclass
class SwapPosition:
    pos1: int
    pos2: int

    def apply(self, password: str) -> str:
        result = list(password)
        result[self.pos1], result[self.pos2] = result[self.pos2], result[self.pos1]
        return "".join(result)

    def unapply(self, password: str) -> str:
        return self.apply(password)


def test_swap_position() -> None:
    sut = SwapPosition(2, 5)

    assert sut.apply("0123456") == "0153426"


@dataclass
class Move:
    from_pos: int
    to_pos: int

    def apply(self, password: str) -> str:
        result = list(password)
        to_move = result[self.from_pos]
        result.remove(to_move)
        result[self.to_pos:self.to_pos] = to_move
        return "".join(result)

    def unapply(self, password: str) -> str:
        return Move(self.to_pos, self.from_pos).apply(password)


def test_move() -> None:
    sut = Move(2, 5)
    assert sut.apply("abcdefgh") == "abdefcgh"
    assert sut.unapply("abdefcgh") == "abcdefgh"

    sut = Move(5, 2)
    assert sut.apply("abcdefg") == "abfcdeg"
    assert sut.unapply("abfcdeg") == "abcdefg"


@dataclass
class SwapLetters:
    letter1: str
    letter2: str

    def apply(self, password: str) -> str:
        result = list(password)
        index1 = result.index(self.letter1)
        index2 = result.index(self.letter2)
        result[index1], result[index2] = result[index2], result[index1]
        return "".join(result)

    def unapply(self, password: str) -> str:
        return self.apply(password)


def test_swap_letters() -> None:
    sut = SwapLetters("X", "Y")
    assert sut.apply("XbcdYfg") == "YbcdXfg"


@dataclass
class RotateRight:
    steps: int

    def apply(self, password: str) -> str:
        result = list(password)
        rotation = self.steps % len(result)
        return "".join(result[-rotation:] + result[:-rotation])

    def unapply(self, password: str) -> str:
        return RotateLeft(self.steps).apply(password)


@dataclass
class RotateLeft:
    steps: int

    def apply(self, password: str) -> str:
        result = list(password)
        rotation = self.steps % len(result)
        return "".join(result[rotation:] + result[:rotation])

    def unapply(self, password: str) -> str:
        return RotateRight(self.steps).apply(password)


type Operation = Reverse | RotateByLetter | SwapPosition | Move | SwapLetters | RotateRight | RotateLeft


def parse_operation(line: str) -> Operation:
    # reverse positions 1 through 6
    if m := re.match(r"reverse positions (\d+) through (\d+)", line):
        return Reverse(int(m[1]), int(m[2]))

    # rotate based on position of letter a
    if m := re.match(r"rotate based on position of letter (.)", line):
        return RotateByLetter(m[1])

    # swap position 4 with position 1
    if m := re.match(r"swap position (\d+) with position (\d+)", line):
        return SwapPosition(int(m[1]), int(m[2]))

    # move position 5 to position 7
    if m := re.match(r"move position (\d+) to position (\d+)", line):
        return Move(int(m[1]), int(m[2]))

    # swap letter d with letter c
    if m:= re.match(r"swap letter (.) with letter (.)", line):
        return SwapLetters(m[1], m[2])

    # rotate right 0 steps
    if m := re.match(r"rotate (right|left) (\d+) steps?", line):
        if m[1] == "right":
            return RotateRight(int(m[2]))
        elif m[1] == "left":
            return RotateLeft(int(m[2]))
        else:
            raise ValueError(f"Invalid rotation: {m[1]}")


    raise ValueError(f"No idea how to parse <{line}>.")


input = read_aoc_input(parse_operation)

password = "abcdefgh"
for op in input:
    password = op.apply(password)
print(password)

password = "fbgdceah"
for op in input[::-1]:
    password = op.unapply(password)
print(password)
