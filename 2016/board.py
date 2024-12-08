class Pos:
    def __init__(self, row: int, col: int, board: "Board"):
        self.row = row
        self.col = col

        self.board = board

    def move(self, direction: str):
        if direction == "U" and self.board.is_valid(self.row - 1, self.col):
            self.row -= 1
        elif direction == "D" and self.board.is_valid(self.row + 1, self.col):
            self.row += 1
        elif direction == "L" and self.board.is_valid(self.row, self.col - 1):
            self.col -= 1
        elif direction == "R" and self.board.is_valid(self.row, self.col + 1):
            self.col += 1


class Board:
    def __init__(self, rows: list[list[str]], row_no: int, col_no: int):
        self.rows = rows
        self.pos = Pos(
            row_no,
            col_no,
            self)
    
    def move(self, direction: str):
        self.pos.move(direction)
    
    def current(self):
        return self.rows[self.pos.row][self.pos.col]
    
    def is_valid(self, row: int, col: int):
        return (
            0 <= row <= len(self.rows) - 1
            and 0 <= col <= len(self.rows[row]) - 1
            and self.rows[row][col] != " ")