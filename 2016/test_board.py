from board import Pos


def test_up():
    pos = Pos(1, 1, 2, 2)
    pos.move("U")

    assert pos.row == 0
    assert pos.col == 1


def test_up_twice():
    """Does not go out of the board."""
    pos = Pos(1, 1, 2, 2)
    pos.move("U")
    pos.move("U")

    assert pos.row == 0
    assert pos.col == 1


def test_down():
    pos = Pos(1, 1, 2, 2)
    pos.move("D")

    assert pos.row == 2
    assert pos.col == 1


def test_up_down_down():
    pos = Pos(1, 1, 2, 2)
    pos.move("U")
    pos.move("D")
    pos.move("D")

    assert pos.row == 2
    assert pos.col == 1


def test_left():
    pos = Pos(1, 1, 2, 2)
    pos.move("L")

    assert pos.row == 1
    assert pos.col == 0


def test_right():
    pos = Pos(1, 1, 2, 2)
    pos.move("R")

    assert pos.row == 1
    assert pos.col == 2


def test_ULDRR():
    pos = Pos(1, 1, 2, 2)
    for dir in "ULDRR":
        pos.move(dir)

    assert pos.row == 1
    assert pos.col == 2
