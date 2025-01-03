from direction import Direction


def test_initial_direction() -> None:
    direction = Direction()
    assert direction == "N"


def test_turn_right() -> None:
    direction = Direction()
    direction.turn("R")
    assert direction == "E"


def test_turn_left() -> None:
    direction = Direction()
    direction.turn("L")
    assert direction == "W"


def test_full_rotation_right() -> None:
    direction = Direction()
    for _ in range(4):
        direction.turn("R")
    assert direction == "N"


def test_full_rotation_left() -> None:
    direction = Direction()
    for _ in range(4):
        direction.turn("L")
    assert direction == "N"


def test_left_twice() -> None:
    direction = Direction()
    direction.turn("L")
    direction.turn("L")

    assert direction == "S"


def test_right_twice() -> None:
    direction = Direction()
    direction.turn("R")
    direction.turn("R")

    assert direction == "S"
