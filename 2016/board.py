class Pos:
    def __init__(self, row: int, col: int, max_row: int, max_col: int):
        self.row = row
        self.col = col

        self.max_row = max_row
        self.max_col = max_col

    def move(self, direction: str):
        if direction == "U" and 0 < self.row:
            self.row -= 1
        elif direction == "D" and self.row < self.max_row:
            self.row += 1
        elif direction == "L" and 0 < self.col:
            self.col -= 1
        elif direction == "R" and self.col < self.max_col:
            self.col += 1


class Board:
    def __init__(self, rows: list[list[str]], row_no: int, col_no: int):
        self.rows = rows
        self.pos = Pos(
            row_no,
            col_no,
            len(self.rows) - 1,
            # Assume all rows are the same length.
            len(self.rows[0]) - 1)
    
    def move(self, direction: str):
        self.pos.move(direction)
    
    def current(self):
        return self.rows[self.pos.row][self.pos.col]