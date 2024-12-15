class Pos:
    def __init__(self, row: int, col: int, board: "Board") -> None:
        self.row = row
        self.col = col

        self.board = board

    def move(self, direction: str) -> None:
        if direction == "U" and self.board.is_valid(self.row - 1, self.col):
            self.row -= 1
        elif direction == "D" and self.board.is_valid(self.row + 1, self.col):
            self.row += 1
        elif direction == "L" and self.board.is_valid(self.row, self.col - 1):
            self.col -= 1
        elif direction == "R" and self.board.is_valid(self.row, self.col + 1):
            self.col += 1


class Board:
    def __init__(self, rows: list[str]) -> None:
        self.rows = rows

    def __getitem__(self, pos: Pos) -> str:
        return self.rows[pos.row][pos.col]

    def pos(self, row: int, col: int) -> Pos:
        return Pos(row, col, self)

    def is_valid(self, row: int, col: int) -> bool:
        return (
            0 <= row <= len(self.rows) - 1
            and 0 <= col <= len(self.rows[row]) - 1
            and self.rows[row][col] != " "
        )
