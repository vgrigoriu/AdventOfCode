from direction import Direction


def test_initial_direction():
    direction = Direction()
    assert str(direction) == "N"


def test_turn_right():
    direction = Direction()
    direction.turn("R")
    assert str(direction) == "E"


def test_turn_left():
    direction = Direction()
    direction.turn("L")
    assert str(direction) == "W"


def test_full_rotation_right():
    direction = Direction()
    for _ in range(4):
        direction.turn("R")
    assert str(direction) == "N"


def test_full_rotation_left():
    direction = Direction()
    for _ in range(4):
        direction.turn("L")
    assert str(direction) == "N"


def test_left_twice():
    direction = Direction()
    direction.turn("L")
    direction.turn("L")

    assert str(direction) == "S"


def test_right_twice():
    direction = Direction()
    direction.turn("R")
    direction.turn("R")

    assert str(direction) == "S"
