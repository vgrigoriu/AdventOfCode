from typing import ClassVar


class Direction:
    compass: ClassVar[list[str]] = ["N", "E", "S", "W"]

    def __init__(self) -> None:
        self.index = 0

    def turn(self, direction: str) -> None:
        if direction == "R":
            self.index = (self.index + 1) % len(Direction.compass)
        elif direction == "L":
            self.index = (self.index - 1) % len(Direction.compass)

    def __eq__(self, s: object) -> bool:
        return Direction.compass[self.index] == s
