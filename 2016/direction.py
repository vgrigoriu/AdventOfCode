class Direction:
    compass = ["N", "E", "S", "W"]

    def __init__(self):
        self.index = 0

    def turn(self, direction: str):
        if direction == "R":
            self.index = (self.index + 1) % len(Direction.compass)
        elif direction == "L":
            self.index = (self.index - 1) % len(Direction.compass)

    def __str__(self):
        return Direction.compass[self.index]
