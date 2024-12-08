from board import Board


def build_pos():
    return Board(["abc", "def", "ghi"]).pos(1, 1)


def test_up():
    pos = build_pos()
    pos.move("U")

    assert pos.row == 0
    assert pos.col == 1


def test_up_twice():
    """Does not go out of the board."""
    pos = build_pos()
    pos.move("U")
    pos.move("U")

    assert pos.row == 0
    assert pos.col == 1


def test_down():
    pos = build_pos()
    pos.move("D")

    assert pos.row == 2
    assert pos.col == 1


def test_up_down_down():
    pos = build_pos()
    pos.move("U")
    pos.move("D")
    pos.move("D")

    assert pos.row == 2
    assert pos.col == 1


def test_left():
    pos = build_pos()
    pos.move("L")

    assert pos.row == 1
    assert pos.col == 0


def test_right():
    pos = build_pos()
    pos.move("R")

    assert pos.row == 1
    assert pos.col == 2


def test_ULDRR():
    pos = build_pos()
    for dir in "ULDRR":
        pos.move(dir)

    assert pos.row == 1
    assert pos.col == 2
